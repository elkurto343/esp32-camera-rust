use std::{
    io::{self, Read},
    net::TcpStream,
};

use crate::ov2460_config::{FrameSize, PixelFormat};

// TODO: replace with protobuf?

#[repr(u8)]
#[derive(Debug)]
pub enum Instruction {
    Capture = 0,
    Format(PixelFormat),
    Resolution(FrameSize),
    Restart,
}

impl Instruction {
    fn from_stream(mut stream: TcpStream) -> io::Result<Self> {
        let mut buf = [0; 5];
        stream.read_exact(&mut buf).unwrap();

        let header = buf[0];
        let payload = &buf[1..5];

        match header {
            0 => Ok(Instruction::Capture),
            1 => {
                let payload: u32 = u32::from_be_bytes(payload.try_into().unwrap());
                let pixel_format = PixelFormat::from(payload);
                Ok(Instruction::Format(pixel_format))
            }
            2 => {
                let payload: u32 = u32::from_be_bytes(payload.try_into().unwrap());
                let frame_size = FrameSize::from(payload);
                Ok(Instruction::Resolution(frame_size))
            }
            3 => Ok(Instruction::Restart),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "message: invalid header",
            )),
        }
    }
}

#[repr(u8)]
#[derive(Debug)]
pub enum InstructionResult {
    Capture(Vec<u8>) = 0,
    Format(bool),
    Resolution(bool),
    Restart(bool),
}

pub fn handle_message(mut stream: TcpStream) -> io::Result<Instruction> {
    println!("tcp: received message from {}", stream.peer_addr().unwrap());

    let message = Instruction::from_stream(stream).unwrap();
    println!("message: {:#?}", message);

    Ok(message)
}
