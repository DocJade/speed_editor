// The resolve speed editor is pointlessly locked down, let's make it useful again.


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
    RELATIVE_0			= 0,		// # Rela
	ABSOLUTE_CONTINUOUS	= 1,		// # Send an "absolute" position (based on the position when mode was set) -4096 -> 4096 range ~ half a turn
	RELATIVE_2			= 2,		// # Same as mode 0 ?
	ABSOLUTE_DEADZERO	= 3,		// # Same as mode 1 but with a small dead band around zero that maps to 0
}

fn main() {
    println!("Hello, world!");
}
