use esp_idf_sys::*;

pub const DEFAULT_PIXEL_FORMAT: PixelFormat = PixelFormat::JPEG;

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
            _ => DEFAULT_PIXEL_FORMAT,
        }
    }
}

impl Default for PixelFormat {
    fn default() -> Self {
        DEFAULT_PIXEL_FORMAT
    }
}
