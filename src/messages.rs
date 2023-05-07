#[derive(Debug)]
pub enum Message {
    Capture,
    Format(PixelFormat),
    Resolution(FrameSize),
    Interval(u8),
    Restart,
}

pub enum Response {
    Capture(Vec<u8>),
    Format(bool),
    Resolution(bool),
    Interval(bool),
    Restart(bool),
}
