use core::convert::From;
use esp_idf_sys::*;
use std::os::raw::c_int;

const DEFAULT_PIXEL_FORMAT: PixelFormat = PixelFormat::JPEG;
const DEFAULT_FRAME_SIZE: FrameSize = FrameSize::SVGA;
const DEFAULT_QUALITY: c_int = 12;

pub const WROVER_PINS: PinAssignments = PinAssignments {
    pwdn_gpio_num: -1,
    reset_gpio_num: -1,
    xclk_gpio_num: 21,
    siod_gpio_num: 26,
    sioc_gpio_num: 27,
    y9_gpio_num: 35,
    y8_gpio_num: 34,
    y7_gpio_num: 39,
    y6_gpio_num: 36,
    y5_gpio_num: 19,
    y4_gpio_num: 18,
    y3_gpio_num: 5,
    y2_gpio_num: 4,
    vsync_gpio_num: 25,
    href_gpio_num: 23,
    pclk_gpio_num: 22,
};

// TODO: consolidate board pins vs camera pins
#[derive(Debug)]
pub struct PinAssignments {
    pwdn_gpio_num: c_int,
    reset_gpio_num: c_int,
    xclk_gpio_num: c_int,
    siod_gpio_num: c_int,
    sioc_gpio_num: c_int,
    y9_gpio_num: c_int,
    y8_gpio_num: c_int,
    y7_gpio_num: c_int,
    y6_gpio_num: c_int,
    y5_gpio_num: c_int,
    y4_gpio_num: c_int,
    y3_gpio_num: c_int,
    y2_gpio_num: c_int,
    vsync_gpio_num: c_int,
    href_gpio_num: c_int,
    pclk_gpio_num: c_int,
}

#[derive(Debug)]
pub enum Board {
    WROVER,
}

impl Board {
    fn pins(&self) -> &PinAssignments {
        match self {
            Board::WROVER => &WROVER_PINS,
        }
    }
}

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

#[derive(Debug)]
pub struct OV2460Config {
    pub pixel_format: PixelFormat,
    pub frame_size: FrameSize,
    pub board: Board,
}

impl Into<camera_config_t> for OV2460Config {
    fn into(self) -> camera_config_t {
        let pins = self.board.pins();
        camera_config_t {
            pin_pwdn: pins.pwdn_gpio_num,
            pin_reset: pins.reset_gpio_num,
            pin_xclk: pins.xclk_gpio_num,
            __bindgen_anon_1: camera_config_t__bindgen_ty_1 {
                pin_sccb_sda: pins.siod_gpio_num,
            },
            __bindgen_anon_2: camera_config_t__bindgen_ty_2 {
                pin_sscb_scl: pins.sioc_gpio_num,
            },
            pin_d7: pins.y9_gpio_num,
            pin_d6: pins.y8_gpio_num,
            pin_d5: pins.y7_gpio_num,
            pin_d4: pins.y6_gpio_num,
            pin_d3: pins.y5_gpio_num,
            pin_d2: pins.y4_gpio_num,
            pin_d1: pins.y3_gpio_num,
            pin_d0: pins.y2_gpio_num,
            pin_vsync: pins.vsync_gpio_num,
            pin_href: pins.href_gpio_num,
            pin_pclk: pins.pclk_gpio_num,
            xclk_freq_hz: 20_000_000,
            ledc_timer: LedcTimer::Timer0.into(),
            ledc_channel: LedcChannel::Channel0.into(),
            pixel_format: self.pixel_format.into(),
            frame_size: self.frame_size.into(),
            jpeg_quality: DEFAULT_QUALITY, // TODO: make configurable
            fb_count: 1,
            ..Default::default()
        }
    }
}

impl Default for OV2460Config {
    fn default() -> Self {
        Self {
            pixel_format: DEFAULT_PIXEL_FORMAT,
            frame_size: DEFAULT_FRAME_SIZE,
            board: Board::WROVER,
        }
    }
}

impl OV2460Config {
    fn new(pixel_format: PixelFormat, frame_size: FrameSize, board: Board) -> Self {
        OV2460Config {
            pixel_format,
            frame_size,
            board,
        }
    }

    pub fn set_pixel_format(&mut self, pixel_format: PixelFormat) {
        self.pixel_format = pixel_format;
    }

    pub fn set_frame_size(&mut self, frame_size: FrameSize) {
        self.frame_size = frame_size;
    }
}
