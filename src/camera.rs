use base64::Engine;
use core::convert::From;
use std::os::raw::c_int;

use esp_idf_sys::*;
use esp_idf_sys::{esp_camera_fb_get, esp_camera_fb_return, esp_camera_init, sensor_t};

use crate::boards::DvpPins;

mod framesize;
mod pixelformat;

pub use framesize::FrameSize;
pub use pixelformat::PixelFormat;

const DEFAULT_JPEG_QUALITY: c_int = 12;

#[derive(Debug)]
pub enum LedcChannel {
    Channel0,
    Channel1,
    Channel2,
    Channel3,
    Channel4,
    Channel5,
    Channel6,
    Channel7,
}

// rust enum -> lib binding
impl From<LedcChannel> for u32 {
    fn from(channel: LedcChannel) -> Self {
        match channel {
            LedcChannel::Channel0 => ledc_channel_t_LEDC_CHANNEL_0,
            LedcChannel::Channel1 => ledc_channel_t_LEDC_CHANNEL_1,
            LedcChannel::Channel2 => ledc_channel_t_LEDC_CHANNEL_2,
            LedcChannel::Channel3 => ledc_channel_t_LEDC_CHANNEL_3,
            LedcChannel::Channel4 => ledc_channel_t_LEDC_CHANNEL_4,
            LedcChannel::Channel5 => ledc_channel_t_LEDC_CHANNEL_5,
            LedcChannel::Channel6 => ledc_channel_t_LEDC_CHANNEL_6,
            LedcChannel::Channel7 => ledc_channel_t_LEDC_CHANNEL_7,
        }
    }
}

#[derive(Debug)]
pub enum LedcTimer {
    Timer0,
    Timer1,
    Timer2,
    Timer3,
}

// rust enum -> lib binding
impl From<LedcTimer> for u32 {
    fn from(timer: LedcTimer) -> Self {
        match timer {
            LedcTimer::Timer0 => ledc_timer_t_LEDC_TIMER_0,
            LedcTimer::Timer1 => ledc_timer_t_LEDC_TIMER_1,
            LedcTimer::Timer2 => ledc_timer_t_LEDC_TIMER_2,
            LedcTimer::Timer3 => ledc_timer_t_LEDC_TIMER_3,
        }
    }
}

pub struct SensorInfo {
    pid: u16,
    name: &'static str,
    max_frame_size: FrameSize,
}

// representation of camera sensor for direct use with esp32-camera library via transform
pub struct CameraSensor {
    pixel_format: PixelFormat,
    frame_size: FrameSize,
    // jpeg_quality: JpegQuality,
    dvp_pins: DvpPins,
}

impl CameraSensor {
    pub fn new(
        pixel_format: Option<PixelFormat>,
        frame_size: Option<FrameSize>,
        // jpeg_quality: Option<JpegQuality>,
        dvp_pins: DvpPins,
    ) -> anyhow::Result<Self> {
        let pixel_format = pixel_format.unwrap_or_default();
        let frame_size = frame_size.unwrap_or_default();

        let pins = dvp_pins;
        let result = unsafe {
            // TODO: remove the need for this
            esp_camera_init(&camera_config_t {
                pin_pwdn: pins.pwdn,
                pin_reset: pins.rst,
                pin_xclk: pins.xclk,
                __bindgen_anon_1: camera_config_t__bindgen_ty_1 {
                    pin_sccb_sda: pins.sda,
                },
                __bindgen_anon_2: camera_config_t__bindgen_ty_2 {
                    pin_sscb_scl: pins.scl,
                },
                pin_d7: pins.d7,
                pin_d6: pins.d6,
                pin_d5: pins.d5,
                pin_d4: pins.d4,
                pin_d3: pins.d3,
                pin_d2: pins.d2,
                pin_d1: pins.d1,
                pin_d0: pins.d0,
                pin_vsync: pins.vsync,
                pin_href: pins.href,
                pin_pclk: pins.pclk,
                xclk_freq_hz: 20_000_000,
                ledc_timer: LedcTimer::Timer0.into(),
                ledc_channel: LedcChannel::Channel0.into(),
                pixel_format: pixel_format.clone().into(),
                frame_size: frame_size.clone().into(),
                jpeg_quality: DEFAULT_JPEG_QUALITY, // TODO: make configurable
                fb_count: 1,
                ..Default::default()
            })
        };

        // let result = unsafe { esp_camera_init(&sensor.into()) };
        match result {
            0 => Ok(CameraSensor {
                pixel_format,
                frame_size,
                // jpeg_quality: jpeg_quality.unwrap_or_default(),
                dvp_pins,
            }),
            // TODO: return error
            _ => panic!("error: failed to init camera"),
        }
    }

    fn get_sensor(&self) -> *mut sensor_t {
        unsafe { esp_camera_sensor_get() }
    }

    // TODO: the sensor functions are added to esp32-camera as conditional includes (via macro)
    // and bindings are not generated for them. Unsure if/how it's possible to expose them. Adding
    // `CONFIG_OV2640_SUPPORT=y` to `sdkconfig.defaults` seems to have no effect.
    // Alternative option is to deinit and re-init every time we want to change the format. 💩
    // Second alternative option is to port those functions to rust and make them sensor agnostic. 🪨

    // pub fn set_pixel_format(&mut self, pixel_format: PixelFormat) {
    //     self.pixel_format = pixel_format;
    //     let mut sensor = unsafe { esp_camera_sensor_get() };
    //     let result = unsafe { (*sensor).set_pixformat(sensor, pixel_format.clone().into()) };
    //     // if result != 0 {}
    // }

    pub fn set_frame_size(&self, framesize: FrameSize) -> Result<(), ()> {
        let sensor = self.get_sensor();
        if let Some(set_framesize) = unsafe { (*sensor).set_framesize } {
            let result = unsafe { set_framesize(sensor, framesize.clone().into()) };
            if result == 0 {
                return Ok(());
            } else {
                return Err(());
            }
        } else {
            return Err(());
        }
    }

    // pub fn set_jpeg_quality(&mut self, jpeg_quality: JpegQuality) {
    //     // min: 0, max: 63
    //     self.jpeg_quality = jpeg_quality;
    // }

    // Capture image using camera module
    pub fn capture_image(&self, debug: bool) -> anyhow::Result<&'static [u8]> {
        // TODO: figure out how to use esp wrapper macros
        // Get the frame buffer from the camera driver
        let fb = unsafe { esp_camera_fb_get() };
        if fb.is_null() {
            // TODO: return error
            eprintln!("error: failed to get camera buffer")
        }

        let img_data = unsafe { std::slice::from_raw_parts((*fb).buf, (*fb).len as usize) };

        if debug == true {
            // Print the base64 encoded image to console for debugging purposes
            let base64_img = base64::engine::general_purpose::STANDARD.encode(img_data);
            println!("----------------------------------------------");
            println!("Image size: {} KB", img_data.len() / 1024);
            println!("----------------------------------------------");
            println!("{}", base64_img);
        }

        // Return the frame buffer to the camera driver
        unsafe { esp_camera_fb_return(fb) };

        Ok(img_data)
    }

    // pub fn sensor_info(self) -> SensorInfo {
    //     let sensor = self.get();
    //     let pid = unsafe { (*sensor).id };
    //     let name = unsafe { (*sensor).name };
    //     let max_frame_size = unsafe { (*sensor).max_size };
    //     SensorInfo {
    //         pid,
    //         name,
    //         max_frame_size,
    //     }
    // }
}
