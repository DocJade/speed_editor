// The resolve speed editor is pointlessly locked down, let's make it useful again.

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

fn main() {
    println!("Hello, world!");
}
