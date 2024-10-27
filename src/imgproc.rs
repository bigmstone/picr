use image::GrayImage;

fn calculate_image_stats(image: &GrayImage) -> (f32, f32, f32) {
    let pixels: Vec<u8> = image.pixels().map(|p| p[0]).collect();

    let mean: f32 = pixels.iter().map(|&v| v as f32).sum::<f32>() / pixels.len() as f32;

    let median = {
        let mut sorted_pixels = pixels.clone();
        sorted_pixels.sort_unstable();
        sorted_pixels[sorted_pixels.len() / 2] as f32
    };

    let std_dev: f32 = (pixels
        .iter()
        .map(|&v| {
            let diff = v as f32 - mean;
            diff * diff
        })
        .sum::<f32>()
        / pixels.len() as f32)
        .sqrt();

    (mean, median, std_dev)
}

pub fn apply_stf_autostretch(image: &mut GrayImage) {
    let (_, median, std_dev) = calculate_image_stats(image);

    let lower_bound = (median - std_dev).max(0.0);
    let upper_bound = (median + std_dev).min(255.0);

    for pixel in image.pixels_mut() {
        let val = pixel[0] as f32;
        let stretched_val = if val < lower_bound {
            0.0
        } else if val > upper_bound {
            255.0
        } else {
            (255.0 * (val - lower_bound) / (upper_bound - lower_bound)).round()
        };
        pixel[0] = stretched_val as u8;
    }
}
