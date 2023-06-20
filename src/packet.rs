use std::{
    io::{self, Read},
    net::TcpStream,
};

use crate::camera::{FrameSize, PixelFormat};

// TODO: consider adding a payload length byte
// Packets for controlling/configuring the ESP32
#[repr(u8)]
#[derive(Debug)]
pub enum IncomingPacket {
    Capture = 1,
    SetPixelFormat(PixelFormat),
    SetFrameSize(FrameSize),
    Restart,
}

impl TryFrom<&mut TcpStream> for IncomingPacket {
    type Error = io::Error;

    // Try deserialize incoming packet from TCPStream
    fn try_from(stream: &mut TcpStream) -> io::Result<Self> {
        let mut buf = [0; 5];
        stream.read_exact(&mut buf).unwrap();

        // Note: the packet format is effectively a header byte used for identifying the packet
        // type followed by optional payload bytes (currently max 4 to match the unsigned 32-bit
        // integers that represent either FrameSize or PixelFormat)
        let header = buf[0];
        let payload = &buf[1..5];

        match header {
            1 => Ok(IncomingPacket::Capture),
            2 => {
                let payload: u32 = u32::from_be_bytes(payload.try_into().unwrap());
                let pixel_format = PixelFormat::from(payload);
                Ok(IncomingPacket::SetPixelFormat(pixel_format))
            }
            3 => {
                let payload: u32 = u32::from_be_bytes(payload.try_into().unwrap());
                let frame_size = FrameSize::from(payload);
                Ok(IncomingPacket::SetFrameSize(frame_size))
            }
            4 => Ok(IncomingPacket::Restart),
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
    Capture(Vec<u8>) = 1,
    SetPixelFormat(bool),
    SetFrameSize(bool),
    Restart(bool),
    // TODO: Error
}

impl Into<Vec<u8>> for OutgoingPacket {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();

        match self {
            OutgoingPacket::Capture(data) => {
                bytes.push(1);
                bytes.extend(data);
            }
            OutgoingPacket::SetPixelFormat(success) => {
                bytes.push(2);
                bytes.push(if success { 1 } else { 0 });
            }
            OutgoingPacket::SetFrameSize(success) => {
                bytes.push(3);
                bytes.push(if success { 1 } else { 0 });
            }
            OutgoingPacket::Restart(success) => {
                bytes.push(4);
                bytes.push(if success { 1 } else { 0 });
            }
        }

        bytes
    }
}
