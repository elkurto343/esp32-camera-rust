use esp_idf_hal::{gpio::PinDriver, peripherals::Peripherals, reset::restart};
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_sys::{self as _, esp_camera_init};

use std::net::TcpListener;

mod ov2460_config;
use ov2460_config::OV2460Config;

mod wifi;
use wifi::wifi;

mod messages;
use messages::{handle_message, Instruction};

mod camera;
use camera::capture_image;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let sysloop = EspSystemEventLoop::take()?;
    let peripherals = Peripherals::take().unwrap();
    let _wifi = wifi(peripherals.modem, sysloop.clone());
    let mut led = PinDriver::output(peripherals.pins.gpio2)?;

    // Initialize the camera
    let mut camera_config = OV2460Config {
        ..Default::default()
    };

    let result = unsafe { esp_camera_init(&camera_config.into()) };
    if result != 0 {
        panic!("Camera initialization failed with error {}", result);
    }

    let listener = TcpListener::bind("0.0.0.0:8080")?;
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let message = handle_message(stream).unwrap();
                match message {
                    Instruction::Capture => {
                        led.set_high()?;
                        capture_image();
                        led.set_low()?;
                    }
                    // Instruction::Resolution(frame_size) => {
                    //     &camera_config.set_frame_size(frame_size);
                    //     // TODO: fix borrow checker issue
                    //     let result = unsafe { esp_camera_init(&camera_config.into()) };
                    //     if result != 0 {
                    //         // TODO: error response
                    //         eprintln!("failed to reconfigure camera");
                    //     } else {
                    //         // TODO: success response
                    //         // println!("camera: config: {:#?}", &camera_config);
                    //     }
                    // }
                    // Instruction::Format(pixel_format) => {
                    //     &camera_config.set_pixel_format(pixel_format);
                    //     // TODO: fix borrow checker issue
                    //     let result = unsafe { esp_camera_init(&camera_config.into()) };
                    //     if result != 0 {
                    //         // TODO: error response
                    //         eprintln!("failed to reconfigure camera");
                    //     } else {
                    //         // TODO: success response
                    //         // println!("camera: config: {:#?}", &camera_config);
                    //     }
                    // }
                    Instruction::Restart => {
                        println!("device: restarting");
                        restart();
                    }
                    _ => (),
                }
            }
            Err(e) => {
                println!("tcp: error {:#?}", e)
            }
        }
    }

    Ok(())
}
