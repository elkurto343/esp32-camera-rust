use std::{
    io::{self, Read},
    net::TcpStream,
};

use crate::camera::{FrameSize, PixelFormat};

// Packets for controlling/configuring the ESP32
#[repr(u8)]
#[derive(Debug)]
pub enum IncomingPacket {
    Capture = 0,
    SetPixelFormat(PixelFormat),
    SetFrameSize(FrameSize),
    Restart,
}

impl TryFrom<TcpStream> for IncomingPacket {
    type Error = io::Error;

    // Deserialize an instruction packet from TCPStream
    fn try_from(mut stream: TcpStream) -> io::Result<Self> {
        let mut buf = [0; 5];
        stream.read_exact(&mut buf).unwrap();

        // Note: the packet format is effectively a header byte used for identifying the packet
        // type followed by optional payload bytes (currently max 4)
        let header = buf[0];
        let payload = &buf[1..5];

        match header {
            0 => Ok(IncomingPacket::Capture),
            1 => {
                let payload: u32 = u32::from_be_bytes(payload.try_into().unwrap());
                let pixel_format = PixelFormat::from(payload);
                Ok(IncomingPacket::SetPixelFormat(pixel_format))
            }
            2 => {
                let payload: u32 = u32::from_be_bytes(payload.try_into().unwrap());
                let frame_size = FrameSize::from(payload);
                Ok(IncomingPacket::SetFrameSize(frame_size))
            }
            3 => Ok(IncomingPacket::Restart),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "message: invalid header",
            )),
        }
    }
}

// TODO: Packet format for ESP32 responses
#[repr(u8)]
#[derive(Debug)]
pub enum OutgoingPacket {
    Capture(Vec<u8>) = 0,
    SetPixelFormat(bool),
    SetFrameSize(bool),
    Restart(bool),
    // TODO: Error
}
