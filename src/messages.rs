use std::{
    io::{self, Read},
    net::TcpStream,
};

use crate::ov2460_config::{FrameSize, PixelFormat};

#[repr(u8)]
#[derive(Debug)]
pub enum Message {
    Capture = 0,
    Format(PixelFormat),
    Resolution(FrameSize),
    Interval(u8),
    Restart,
}

impl Message {
    fn from_stream(mut stream: TcpStream) -> io::Result<Self> {
        let mut buf = [0; 5];
        stream.read_exact(&mut buf).unwrap();

        let header = buf[0];
        let payload = &buf[1..5];

        match header {
            0 => Ok(Message::Capture),
            1 => {
                let payload: u32 = u32::from_be_bytes(payload.try_into().unwrap());
                let pixel_format = PixelFormat::from(payload);
                Ok(Message::Format(pixel_format))
            }
            2 => {
                let payload: u32 = u32::from_be_bytes(payload.try_into().unwrap());
                let frame_size = FrameSize::from(payload);
                Ok(Message::Resolution(frame_size))
            }
            3 => Ok(Message::Interval(payload[0])),
            4 => Ok(Message::Restart),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "message: invalid header",
            )),
        }
    }
}
#[repr(u8)]
#[derive(Debug)]
pub enum Response {
    Capture(Vec<u8>) = 0,
    Format(bool),
    Resolution(bool),
    Interval(bool),
    Restart(bool),
}

pub fn handle_message(mut stream: TcpStream) -> io::Result<Message> {
    println!("tcp: received message from {}", stream.peer_addr().unwrap());

    let message = Message::from_stream(stream).unwrap();
    println!("message: {:#?}", message);

    Ok(message)
}
