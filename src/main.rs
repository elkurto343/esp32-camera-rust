use esp_idf_hal::{gpio::PinDriver, peripherals::Peripherals};
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_sys::{self as _, esp_camera_init};

use std::net::TcpListener;

mod ov2460_config;
use ov2460_config::ov2460_config;

mod wifi;
use wifi::wifi;

mod messages;
use messages::{handle_message, Message};

mod camera;
use camera::capture_image;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let sysloop = EspSystemEventLoop::take()?;
    let peripherals = Peripherals::take().unwrap();
    let wifi = wifi(peripherals.modem, sysloop.clone());
    let mut led = PinDriver::output(peripherals.pins.gpio2)?;

    // Initialize the camera
    // TODO: toggle frame size and capture interval
    let result = unsafe { esp_camera_init(&ov2460_config(None, None)) };
    if result != 0 {
        panic!("Camera initialization failed with error {}", result);
    }

    let listener = TcpListener::bind("0.0.0.0:8080")?;
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let message = handle_message(stream).unwrap();
                match message {
                    Message::Capture => {
                        led.set_high()?;
                        capture_image();
                        led.set_low();
                        continue;
                    }
                    _ => continue, // TODO: other commands
                }
            }
            Err(e) => {
                println!("tcp: error {:#?}", e)
            }
        }
    }

    Ok(())
}
