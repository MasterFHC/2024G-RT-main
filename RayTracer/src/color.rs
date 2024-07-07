use image::RgbImage;
use crate::Vec3;
use crate::intervals::Interval;

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component <= 0.0 {
        0.0
    } else {
        linear_component.sqrt()
    }
}

/// the multi-sample write_color() function
pub fn write_color(pixel_color: Vec3, img: &mut RgbImage, i: usize, j: usize) {
    let pixel = img.get_pixel_mut(i.try_into().unwrap(), j.try_into().unwrap());

    let intensity: Interval = Interval::new(0.000, 0.999);

    // Gamma 2 correction
    let pixel_color = Vec3::new(linear_to_gamma(pixel_color.x), linear_to_gamma(pixel_color.y), linear_to_gamma(pixel_color.z));

    let pixel_color = Vec3::new(255.0 * intensity.clamp(pixel_color.x), 255.0 * intensity.clamp(pixel_color.y), 255.0 * intensity.clamp(pixel_color.z));
    *pixel = image::Rgb([pixel_color.x as u8, pixel_color.y as u8, pixel_color.z as u8]);
    // Write the translated [0,255] value of each color component.

}
