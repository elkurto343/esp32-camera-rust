use base64::Engine;

use esp_idf_sys::{esp_camera_fb_get, esp_camera_fb_return};

pub fn capture_image() {
    // Get the frame buffer from the camera driver
    let fb = unsafe { esp_camera_fb_get() };
    if fb.is_null() {
        eprintln!("camera: failed to capture an image");
    }

    // Base64 encode the image data
    let img_data = unsafe { std::slice::from_raw_parts((*fb).buf, (*fb).len as usize) };
    let base64_img = base64::engine::general_purpose::STANDARD.encode(img_data);

    // TODO: return image bytes and ditch base64
    // Print the base64 encoded image to console
    println!("----------------------------------------------");
    println!("Image size: {} KB", img_data.len() / 1024);
    println!("----------------------------------------------");
    println!("{}", base64_img);

    // Return the frame buffer to the camera driver
    unsafe { esp_camera_fb_return(fb) };
}
