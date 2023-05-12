use esp_idf_hal::{gpio::PinDriver, peripherals::Peripherals};
use esp_idf_svc;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_sys::{self as _, esp_camera_fb_get, esp_camera_fb_return, esp_camera_init};

use std::thread;
use std::time::Duration;

use base64::Engine;

mod ov2460_config;
use ov2460_config::ov2460_config;

mod wifi;
use wifi::wifi;

// const LED_BUILTIN = 2;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let sysloop = EspSystemEventLoop::take()?;
    let peripherals = Peripherals::take().unwrap();
    let mut wifi = wifi(peripherals.modem, sysloop.clone());

    // TODO: how to make this use constant
    let mut led = PinDriver::output(peripherals.pins.gpio2)?;

    // Initialize the camera
    // TODO: toggle frame size and capture interval
    let result = unsafe { esp_camera_init(&ov2460_config(None, None)) };
    if result != 0 {
        panic!("Camera initialization failed with error {}", result);
    }

    loop {
        // TODO: listen to TCP for messages

        led.set_high()?;

        // Capture an image
        let fb = unsafe { esp_camera_fb_get() };
        if fb.is_null() {
            eprintln!("Failed to capture an image");
            thread::sleep(Duration::from_secs(5));
            continue;
        }

        // Base64 encode the image data
        let img_data = unsafe { std::slice::from_raw_parts((*fb).buf, (*fb).len as usize) };
        let base64_img = base64::engine::general_purpose::STANDARD.encode(img_data);

        // Print the base64 encoded image to console
        println!("----------------------------------------------");
        println!("Image size: {} KB", img_data.len() / 1024);
        println!("----------------------------------------------");
        println!("{}", base64_img);

        // Return the frame buffer to the camera driver
        unsafe { esp_camera_fb_return(fb) };

        led.set_low()?;

        // Wait for 5 seconds
        thread::sleep(Duration::from_secs(15));
    }
}
