// The resolve speed editor is pointlessly locked down, let's make it useful again.

use std::collections::HashSet;

use rusb::*;

// # Key Presses are reported in Input Report ID 4 as an array of 6 LE16 keycodes
// # that are currently being held down. 0x0000 is no key. No auto-repeat, no hw
// # detection of the 'fast double press'. Every time the set of key being held
// # down changes, a new report is sent.

// no auto repeat is nice
enum SpeedEditorKey {
    None = 0x00,
    // top left cluster
    SmartInsrt = 0x01,
    Appnd = 0x02,
    RiplOwr = 0x03,
    CloseUp = 0x04,
    PlaceOnTop = 0x05,
    SrcOwr = 0x06,
    // bottom left cluster
    In = 0x07,
    Out = 0x08,
    TrimIn = 0x09,
    TrimOut = 0x0a,
    Roll = 0x0b,
    SlipSrc = 0x0c,
    SlipDest = 0x0d,
    TransDur = 0x0e,
    Cut = 0x0f,
    Dis = 0x10,
    SmthCut = 0x11,
    // top right cluster
    Source = 0x1a,
    Timeline = 0x1b,
    // bottom right cluster
    Shtl = 0x1c,
    Jog = 0x1d,
    Scrl = 0x1e,
    // middle top cluster
    Esc = 0x31,
    SyncBin = 0x1f,
    AudioLevel = 0x2c,
    FullView = 0x2d,
    Trans = 0x22,
    Split = 0x2f,
    Snap = 0x2e,
    RiplDel = 0x2b,
    // middle bottom cluster
    Cam1 = 0x33,
    Cam2 = 0x34,
    Cam3 = 0x35,
    Cam4 = 0x36,
    Cam5 = 0x37,
    Cam6 = 0x38,
    Cam7 = 0x39,
    Cam8 = 0x3a,
    Cam9 = 0x3b,
    LiveOwr = 0x30,
    VideoOnly = 0x25,
    AudioOnly = 0x26,
    StopPlay = 0x3c,
}

// # Setting the leds is done with SET_REPORT on Output Report ID 2
// # which takes a single LE32 bitfield of the LEDs to enable

// I like your funny words magic man.
enum SpeedEditorLed {
    // top left cluster
	CloseUp	= (1 <<  0),
    // bottom left cluster
	Cut			= (1 <<  1),
	Dis			= (1 <<  2),
	SmthCut	= (1 <<  3),
    // middle top cluster
	Trans		= (1 <<  4),
	Snap		= (1 <<  5),
    // middle bottom cluster
	Cam7		= (1 <<  6),
	Cam8		= (1 <<  7),
	Cam9		= (1 <<  8),
	LiveOwr	= (1 <<  9),
	Cam4		= (1 << 10),
	Cam5		= (1 << 11),
	Cam6		= (1 << 12),
	VideoOnly	= (1 << 13),
	Cam1		= (1 << 14),
	Cam2		= (1 << 15),
	Cam3		= (1 << 16),
	AudioOnly	= (1 << 17),
}

// # The LEDs for the Jog mode button are on a different system ...
// # Setting those leds is done with SET_REPORT on Output Report ID 4
// # which takes a single 8 bits bitfield of the LEDs to enable

// i know some of these words.

enum SpeedEditorJogLed {
    // bottom right cluster
    Jog			= (1 <<  0),
	Shtl		= (1 <<  1),
	Scrl		= (1 <<  2),
}

enum SpeedEditorJogMode {
    Relative0			= 0,		// # Rela
	AbsoluteContinuous	= 1,		// # Send an "absolute" position (based on the position when mode was set) -4096 -> 4096 range ~ half a turn
	Relative2			= 2,		// # Same as mode 0 ?
	AbsoluteDeadzero	= 3,		// # Same as mode 1 but with a small dead band around zero that maps to 0
}

// now we have the function that authenticates the speed editor

fn bmd_kbd_auth(challenge: u64) -> u64 {

    // sadly the auth implementation isn't commented, so i have no idea
    // how it works.

    fn rol8(v: u64) -> u64 {
        (v << 56) | (v >> 8)
    }

    fn rol8n(v: u64, n: u64) -> u64 {
        let mut mut_v = v;
        for _ in 0..n {
            mut_v = rol8(mut_v)
        }
        mut_v
    }

    const AUTH_EVEN_TBL: [u64; 8] = [
        0x3ae1206f97c10bc8,
        0x2a9ab32bebf244c6,
        0x20a6f8b8df9adf0a,
        0xaf80ece52cfc1719,
        0xec2ee2f7414fd151,
        0xb055adfd73344a15,
        0xa63d2e3059001187,
        0x751bf623f42e0dde
    ];

    const AUTH_ODD_TBL: [u64; 8] = [
        0x3e22b34f502e7fde,
		0x24656b981875ab1c,
		0xa17f3456df7bf8c3,
		0x6df72e1941aef698,
		0x72226f011e66ab94,
		0x3831a3c606296b42,
		0xfd7ff81881332c89,
		0x61a3f6474ff236c6,
    ];

    const MASK: u64 = 0xa79a63f585d37bf0;

    let n: u64 = challenge & 7;
    let mut v: u64 = rol8n(challenge, n);
    let k: u64;

    if (v & 1) == ((0x78 >> n) & 1){
        k = AUTH_EVEN_TBL[n as usize];
    }
    else {
        v = v ^ rol8(v);
		k = AUTH_ODD_TBL[n as usize];
    }

    v ^ (rol8(v) & MASK) ^ k


}

// now onto the editor "class"-es

trait SpeedEditorHandler {
    fn jog(&mut self, mode: SpeedEditorJogMode, value: i32);
    fn key(&mut self, keys: Vec<SpeedEditorKey>);
    fn battery(&mut self, charging: bool, level: u8);
}

const VENDOR_ID: u16 = 0x1edb;
const PRODUCT_ID: u16 = 0xda0e;

struct SpeedEditor{
    device: Device<rusb::Context>,
    handler: Box<dyn SpeedEditorHandler>,
}

struct MySpeedEditorHandler {
    pressed_keys: HashSet<SpeedEditorKey>,
}

impl SpeedEditorHandler for MySpeedEditorHandler {
    fn jog(&mut self, _mode: SpeedEditorJogMode, _value: i32) {
        // Implement logic to handle jog wheel movement, e.g., printing a message
        println!("Jog wheel rotated in mode {:?} with value {}", _mode, _value);
    }

    fn key(&mut self, keys: Vec<SpeedEditorKey>) {
        // Update the pressed keys and implement logic based on pressed/released keys
        self.pressed_keys.extend(keys.iter().cloned());
        let mut pressed_key_names = Vec::new();
        for key in self.pressed_keys.iter() {
            pressed_key_names.push(format!("{:?}", key));
        }
        println!("Keys pressed: {:?}", pressed_key_names);
    }

    fn battery(&mut self, _charging: bool, _level: u8) {
        // Implement logic to handle battery status changes, e.g., printing a message
        println!("Battery charging: {}, level: {}", _charging, _level);
    }
}


impl SpeedEditor {
    fn new(context: rusb::Context) -> Result<Self> {
        let context = Context::new().unwrap();

        let device = context.open_device_with_vid_pid(VENDOR_ID, PRODUCT_ID).unwrap().device();


        let device_handle = device.open()?;
        let interface = device_handle.active_configuration()?.interface(0)?;
    }
}


fn main() {
    println!("Hello, world!");
}
