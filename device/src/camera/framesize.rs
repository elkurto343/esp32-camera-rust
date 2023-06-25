use esp_idf_sys::esp_camera::*;

pub const DEFAULT_FRAME_SIZE: FrameSize = FrameSize::SVGA;

// TODO: implement traits for comparing frame sizes
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
            _ => DEFAULT_FRAME_SIZE,
        }
    }
}

impl Default for FrameSize {
    fn default() -> Self {
        DEFAULT_FRAME_SIZE
    }
}
