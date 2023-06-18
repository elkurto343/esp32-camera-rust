use esp_idf_hal::{gpio::PinDriver, peripherals::Peripherals, reset::restart};
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_sys::{self as _};

use std::net::TcpListener;
use std::{io::Write, num};

mod board;
mod camera;
mod packet;
mod wifi;

use board::Board;
use camera::CameraSensor;
use packet::{IncomingPacket, OutgoingPacket};
use wifi::init_wifi;

const PACKET_CHUNK_SIZE: usize = 1400;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    // TODO: `Board::from_env()` can load board variant and potentially custom pin assignments
    // internally
    let board = env!("ESP32_BOARD");
    let board = match board {
        "Freenove" => Board::Freenove,
        "AIThinker" => Board::AIThinker,
        // TODO: "Custom"
        _ => panic!("env var: invalid board specified"),
    };

    // Initialize general hardware
    let sysloop = EspSystemEventLoop::take()?;
    let peripherals = Peripherals::take().unwrap();
    let mut led = match board {
        Board::Freenove => Some(PinDriver::output(peripherals.pins.gpio2).unwrap()),
        _ => None,
    };

    // Initialize wifi
    let wifi_ssid = env!("WIFI_SSID");
    let wifi_pass = env!("WIFI_PASS");
    let _wifi = init_wifi(wifi_ssid, wifi_pass, peripherals.modem, sysloop.clone());

    // TODO: let Board handle camera instantiation
    // Initialize the camera with default config
    let camera_sensor = CameraSensor::new(None, None, board.dvp_pins()).unwrap();

    // Listen to TCP for instruction packets
    let listener = TcpListener::bind("0.0.0.0:8080")?;
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("tcp: received packet from {}", stream.peer_addr().unwrap());
                let packet = IncomingPacket::try_from(&mut stream);
                println!("tcp: packet: {:#?}", packet);

                // TODO: encapsulate instruction handlers
                match packet {
                    Ok(IncomingPacket::Capture) => {
                        // Capture image while illuminating specified LED
                        // led.unwrap().set_high()?;
                        let image = camera_sensor.capture_image(true).unwrap();
                        // led.unwrap().set_low()?;

                        for chunk in image.chunks(PACKET_CHUNK_SIZE) {
                            let write_result = stream.write(chunk);
                            match write_result {
                                Ok(num_bytes) => println!("tcp: wrote packet: {} bytes", num_bytes),
                                Err(e) => {
                                    eprintln!("error: tcp: failed to write packet {:#?}", e)
                                }
                            }
                        }

                        stream.flush();
                    }
                    Ok(IncomingPacket::SetFrameSize(frame_size)) => {
                        println!("todo: set resolution")
                    }
                    Ok(IncomingPacket::SetPixelFormat(pixel_format)) => {
                        println!("todo: set pixel format")
                    }
                    Ok(IncomingPacket::Restart) => {
                        println!("device: restarting"); // When in doubt.. restart your way out
                        restart();
                    }
                    Err(err) => println!("error parsing packet: {:#?}", err),
                }
            }
            Err(e) => {
                println!("tcp: error {:#?}", e)
            }
        }
    }

    Ok(())
}
