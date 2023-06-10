use esp_idf_hal::{gpio::PinDriver, peripherals::Peripherals, reset::restart};
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_sys::{self as _};

use std::net::TcpListener;

mod wifi;
use wifi::wifi;

mod messages;
use messages::{handle_message, Instruction};

mod camera;
use camera::CameraSensor;

mod board;
use board::Board;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    // Initialize general hardware
    let sysloop = EspSystemEventLoop::take()?;
    let peripherals = Peripherals::take().unwrap();
    let mut led = PinDriver::output(peripherals.pins.gpio2)?; // board's builtin LED
    let board = Board::Freenove;

    // Initialize wifi
    let _wifi = wifi(peripherals.modem, sysloop.clone());

    // TODO: let Board handle camera instantiation
    // Initialize the camera with default config
    let camera_sensor = CameraSensor::new(None, None, board.dvp_pins()).unwrap();

    // Listen to TCP for instruction packets
    let listener = TcpListener::bind("0.0.0.0:8080")?;
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let message = handle_message(stream);
                // TODO: encapsulate instruction handlers
                match message {
                    Err(err) => println!("error parsing packet: {:#?}", err),
                    Ok(Instruction::Capture) => {
                        // Turn LED on while image is being captured
                        led.set_high()?;
                        // Capture image using camera
                        let _image = camera_sensor.capture_image(true);
                        // Turn LED back off after image capture completes
                        led.set_low()?;
                    }
                    // Ok(Instruction::Resolution(frame_size)) => {
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
                    // Ok(Instruction::Format(pixel_format)) => {
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
                    Ok(Instruction::Restart) => {
                        // When in doubt.. restart your way out
                        println!("device: restarting");
                        restart();
                    }
                    _ => (), // commented instructions
                }
            }
            Err(e) => {
                println!("tcp: error {:#?}", e)
            }
        }
    }

    Ok(())
}
