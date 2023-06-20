use std::io::Write;
use std::net::TcpListener;

use esp_idf_hal::{peripherals::Peripherals, reset::restart};
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_sys::{self as _};

mod boards;
mod camera;
mod packet;
mod wifi;

use boards::Board;
use camera::CameraSensor;
use packet::IncomingPacket;
use wifi::init_wifi;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    // Initialize general hardware
    let sysloop = EspSystemEventLoop::take()?;
    let peripherals = Peripherals::take().unwrap();
    let board = Board::from_env();

    // Initialize wifi
    // TODO: encrypt secrets in binary
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
                        let image = camera_sensor.capture_image(true).unwrap();
                        stream.write_all(image);
                        stream.flush();
                    }
                    Ok(IncomingPacket::SetFrameSize(frame_size)) => {
                        println!("set: frame size: {:#?}", frame_size);
                        let result = if let Ok(_) = camera_sensor.set_frame_size(frame_size) {
                            true
                        } else {
                            false
                        };
                        stream.write(result);
                        stream.flush();
                    }
                    Ok(IncomingPacket::SetPixelFormat(pixel_format)) => {
                        println!("todo: set pixel format");
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
