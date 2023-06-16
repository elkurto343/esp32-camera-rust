use esp_idf_hal::{gpio::PinDriver, peripherals::Peripherals, reset::restart};
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_sys::{self as _};

use std::net::TcpListener;

mod board;
mod camera;
mod packet;
mod wifi;

use board::Board;
use camera::CameraSensor;
use packet::IncomingPacket;
use wifi::init_wifi;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    // Initialize general hardware
    let sysloop = EspSystemEventLoop::take()?;
    let peripherals = Peripherals::take().unwrap();
    // TODO: not all boards will have builtin led
    let mut led = PinDriver::output(peripherals.pins.gpio2)?; // board's builtin LED
    let board = Board::Freenove;

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
                let packet = IncomingPacket::try_from(stream);
                println!("packet: {:#?}", packet);

                // TODO: encapsulate instruction handlers
                match packet {
                    Err(err) => println!("error parsing packet: {:#?}", err),
                    Ok(IncomingPacket::Capture) => {
                        // Turn LED on while image is being captured
                        led.set_high()?;
                        // Capture image using camera
                        let _image = camera_sensor.capture_image(true);
                        // Turn LED back off after image capture completes
                        led.set_low()?;
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
                }
            }
            Err(e) => {
                println!("tcp: error {:#?}", e)
            }
        }
    }

    Ok(())
}
