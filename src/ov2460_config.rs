use core::convert::From;
use esp_idf_sys::*;
use std::os::raw::c_int;

// Pin Assignment for ESP32 WROVER (Freenove)

pub const PWDN_GPIO_NUM: c_int = -1;
pub const RESET_GPIO_NUM: c_int = -1;
pub const XCLK_GPIO_NUM: c_int = 21;
pub const SIOD_GPIO_NUM: c_int = 26;
pub const SIOC_GPIO_NUM: c_int = 27;
pub const Y9_GPIO_NUM: c_int = 35;
pub const Y8_GPIO_NUM: c_int = 34;
pub const Y7_GPIO_NUM: c_int = 39;
pub const Y6_GPIO_NUM: c_int = 36;
pub const Y5_GPIO_NUM: c_int = 19;
pub const Y4_GPIO_NUM: c_int = 18;
pub const Y3_GPIO_NUM: c_int = 5;
pub const Y2_GPIO_NUM: c_int = 4;
pub const VSYNC_GPIO_NUM: c_int = 25;
pub const HREF_GPIO_NUM: c_int = 23;
pub const PCLK_GPIO_NUM: c_int = 22;

#[derive(Debug)]
pub enum PixelFormat {
    GRAYSCALE, // Grayscale, 1 byte per pixel
    RGB565,    // RGB565, 2 bytes per pixel
    YUV422,    // YUV422, 2 bytes per pixel
    JPEG,      // JPEG, compressed image format
}

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

#[derive(Debug)]
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
    fn from(frame_rate: FrameSize) -> Self {
        match frame_rate {
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

pub fn ov2460_config(
    pixel_format: Option<PixelFormat>,
    frame_size: Option<FrameSize>,
) -> camera_config_t {
    let default_pixel_format = PixelFormat::JPEG;
    let default_frame_size = FrameSize::SVGA;
    camera_config_t {
        pin_pwdn: PWDN_GPIO_NUM,
        pin_reset: RESET_GPIO_NUM,
        pin_xclk: XCLK_GPIO_NUM,
        __bindgen_anon_1: camera_config_t__bindgen_ty_1 {
            pin_sccb_sda: SIOD_GPIO_NUM,
        },
        __bindgen_anon_2: camera_config_t__bindgen_ty_2 {
            pin_sscb_scl: SIOC_GPIO_NUM,
        },
        pin_d7: Y9_GPIO_NUM,
        pin_d6: Y8_GPIO_NUM,
        pin_d5: Y7_GPIO_NUM,
        pin_d4: Y6_GPIO_NUM,
        pin_d3: Y5_GPIO_NUM,
        pin_d2: Y4_GPIO_NUM,
        pin_d1: Y3_GPIO_NUM,
        pin_d0: Y2_GPIO_NUM,
        pin_vsync: VSYNC_GPIO_NUM,
        pin_href: HREF_GPIO_NUM,
        pin_pclk: PCLK_GPIO_NUM,
        xclk_freq_hz: 20_000_000,
        ledc_timer: LedcTimer::Timer0.into(),
        ledc_channel: LedcChannel::Channel0.into(),
        pixel_format: pixel_format.unwrap_or(default_pixel_format).into(),
        frame_size: frame_size.unwrap_or(default_frame_size).into(),
        jpeg_quality: 12,
        fb_count: 1,
        ..Default::default()
    }
}
