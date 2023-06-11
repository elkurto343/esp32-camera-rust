use base64::Engine;
use core::convert::From;
use std::os::raw::c_int;

use esp_idf_sys::*;
use esp_idf_sys::{esp_camera_fb_get, esp_camera_fb_return, esp_camera_init};

use crate::board::DvpPins;

const DEFAULT_PIXEL_FORMAT: PixelFormat = PixelFormat::JPEG;
const DEFAULT_FRAME_SIZE: FrameSize = FrameSize::SVGA;
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

// supported pixel formats encapsulated in a Rust enum with transforms
#[derive(Debug, Clone)]
pub enum PixelFormat {
    GRAYSCALE, // Grayscale, 1 byte per pixel
    RGB565,    // RGB565, 2 bytes per pixel
    YUV422,    // YUV422, 2 bytes per pixel
    JPEG,      // JPEG, compressed image format
}

// rust enum -> lib binding
impl From<PixelFormat> for u32 {
    fn from(pixel_format: PixelFormat) -> Self {
        match pixel_format {
            PixelFormat::GRAYSCALE => pixformat_t_PIXFORMAT_GRAYSCALE,
            PixelFormat::RGB565 => pixformat_t_PIXFORMAT_RGB565,
            PixelFormat::YUV422 => pixformat_t_PIXFORMAT_YUV422,
            PixelFormat::JPEG => pixformat_t_PIXFORMAT_JPEG,
        }
    }
}

// lib binding -> rust enum
impl From<u32> for PixelFormat {
    fn from(value: u32) -> Self {
        match value {
            pixformat_t_PIXFORMAT_GRAYSCALE => PixelFormat::GRAYSCALE,
            pixformat_t_PIXFORMAT_RGB565 => PixelFormat::RGB565,
            pixformat_t_PIXFORMAT_YUV422 => PixelFormat::YUV422,
            pixformat_t_PIXFORMAT_JPEG => PixelFormat::JPEG,
            _ => panic!("Invalid value for PixelFormat"),
        }
    }
}

impl Default for PixelFormat {
    fn default() -> Self {
        DEFAULT_PIXEL_FORMAT
    }
}

// TODO: refactor/extend to support OV5640
// ESP32 supported image resolutions encapsulated in a Rust enum with transforms
#[derive(Debug, Clone)]
pub enum FrameSize {
    QQVGA, // Quarter Quarter VGA, 160 x 120, 28.8 kbps
    QCIF,  // Quarter Common Intermediate Format, 176 x 144, 38.016 kbps
    QVGA,  // Quarter VGA, 320 x 240, 115.2 kbps
    CIF,   // Common Intermediate Format, 352 x 288, 152.064 kbps
    VGA,   // Video Graphics Array, 640 x 480, 460.8 kbps
    SVGA,  // Super VGA, 800 x 600, 720 kbps
    XGA,   // Extended Graphics Array, 1024 x 768, 1.175296 Mbps
    SXGA,  // Super Extended Graphics Array, 1280 x 1024, 1.96608 Mbps
    UXGA,  // Ultra Extended Graphics Array, 1600 x 1200, 2.88064 Mbps
}

impl From<FrameSize> for u32 {
    fn from(frame_size: FrameSize) -> Self {
        match frame_size {
            FrameSize::QQVGA => framesize_t_FRAMESIZE_QQVGA,
            FrameSize::QCIF => framesize_t_FRAMESIZE_QCIF,
            FrameSize::QVGA => framesize_t_FRAMESIZE_QVGA,
            FrameSize::CIF => framesize_t_FRAMESIZE_CIF,
            FrameSize::VGA => framesize_t_FRAMESIZE_VGA,
            FrameSize::SVGA => framesize_t_FRAMESIZE_SVGA,
            FrameSize::XGA => framesize_t_FRAMESIZE_XGA,
            FrameSize::SXGA => framesize_t_FRAMESIZE_SXGA,
            FrameSize::UXGA => framesize_t_FRAMESIZE_UXGA,
        }
    }
}

impl From<u32> for FrameSize {
    fn from(value: u32) -> Self {
        match value {
            framesize_t_FRAMESIZE_QQVGA => FrameSize::QQVGA,
            framesize_t_FRAMESIZE_QCIF => FrameSize::QCIF,
            framesize_t_FRAMESIZE_QVGA => FrameSize::QVGA,
            framesize_t_FRAMESIZE_CIF => FrameSize::CIF,
            framesize_t_FRAMESIZE_VGA => FrameSize::VGA,
            framesize_t_FRAMESIZE_SVGA => FrameSize::SVGA,
            framesize_t_FRAMESIZE_XGA => FrameSize::XGA,
            framesize_t_FRAMESIZE_SXGA => FrameSize::SXGA,
            framesize_t_FRAMESIZE_UXGA => FrameSize::UXGA,
            _ => panic!("Invalid value for FrameSize"),
        }
    }
}

impl Default for FrameSize {
    fn default() -> Self {
        DEFAULT_FRAME_SIZE
    }
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

    // TODO: the sensor functions are added to esp32-camera as conditional includes (via macro)
    // and bindings are not generated for them. Unsure if/how it's possible to expose them. Adding
    // `CONFIG_OV2640_SUPPORT=y` to `sdkconfig.defaults` seems to have no effect.
    // Alternative option is to deinit and re-init every time we want to change the format.

    // pub fn set_pixel_format(&mut self, pixel_format: PixelFormat) {
    //     self.pixel_format = pixel_format;
    //     let mut sensor = unsafe { esp_camera_sensor_get() };
    //     let result = unsafe { (*sensor).set_pixformat(sensor, pixel_format.clone().into()) };
    //     // if result != 0 {}
    // }

    // pub fn set_frame_size(&mut self, framesize: FrameSize) {
    //     self.frame_size = framesize;
    //     let sensor: *mut sensor_t = unsafe { esp_camera_sensor_get() };
    //
    //     let result = unsafe { set_framesize(*sensor, framesize.clone().into()) };
    //     // if result != 0 {}
    //
    //     // if self.pixel_format == PixelFormat::JPEG {
    //     // let result = unsafe { *sensor.set_framesize(sensor, framesize.into()) };
    //     // Handle the result if needed
    //     // }
    // }

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
}
