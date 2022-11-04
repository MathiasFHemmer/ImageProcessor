use image::{DynamicImage, GenericImageView};

pub fn psnr(original_image: &DynamicImage, processed_image: &DynamicImage) -> f64{
    let dimensions = original_image.dimensions();

    let max_intensity = f64::from(max_intensity(&original_image));
    let avarage_factor = 1.0 / (((dimensions.0-1) as f64 * (dimensions.1-1) as f64)) as f64;

    let mut sum = 0.0;
    for (x, y, o_pixel) in original_image.pixels(){
        let p_pixel = processed_image.get_pixel(x, y);
        sum += (o_pixel[0] as f64 - p_pixel[0] as f64).powf(2.0);
    }
    let mse = sum * avarage_factor;
    let ratio = max_intensity / mse;

    return 10.0 * ratio.log10();
}

fn max_intensity(image: &DynamicImage) -> u32 {
    return u32::pow(u32::from(image.color().bits_per_pixel()) , 2);
}