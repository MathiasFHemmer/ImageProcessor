use fft2d::slice::{fft_2d, fftshift, ifftshift, ifft_2d};
use image::{RgbImage, GenericImageView, Pixel, Rgb, DynamicImage, ImageBuffer};
use rand_distr::num_traits::Pow;
use rustfft::num_complex::Complex;

use crate::{commands::image::filter::*, DATABASE, util};

pub fn handle(args: FilterArgs){
    match args.commands {
        FilterCommands::Median(args) => median(args),
        FilterCommands::LowPass(args) => low_pass(args),
        FilterCommands::LowPassSmooth(args) => low_pass_smooth(args),
    }
}

fn median(args: MedianArgs){
    let mut db = DATABASE.lock().unwrap();

    let range = (args.kernel_size.floor() / 2f32).floor() as u32;
    
    let image = &db.get_history().unwrap().entries.last().unwrap().image;
    let dimension = image.dimensions();
    let mut filtered_image = RgbImage::new(dimension.0, dimension.1);

    for y in 0..dimension.1{
        for x in 0..dimension.0{
            if x > (range-1) && x < (dimension.0 -range) && y > (range-1) && y < (dimension.1 -range){
                filtered_image.put_pixel(x, y, median_pixel(&image, range as i32, x, y));
            } else{
                let current_pixel = image.get_pixel(x, y);
                let pix = current_pixel.clone().to_rgb();
                filtered_image.put_pixel(x, y, pix);
            }
        }
    }

    fn median_pixel(image: &DynamicImage, range: i32, start_x: u32, start_y: u32) -> Rgb<u8>{
        let mut values: Vec<u8> = Vec::new();
        for y in (-range)..(range + 1){
            for x in (-range)..(range + 1){
                let image_x = (start_x as i32 + x) as u32;
                let image_y = (start_y as i32 + y) as u32;
                values.push(
                    image.get_pixel(image_x, image_y).to_rgb()[0]
                );
            }
        }
    
        values.sort();
        let median = (values.len() as f32 / 2.0).floor() as usize;
        let value = values[median];
    
        Rgb([value, value, value])
    }

    let filtered_image = DynamicImage::ImageRgb8(filtered_image);

    let first_image = &db.get_history().unwrap().entries.first().unwrap().image;
    let psnr = util::psnr(first_image, &filtered_image);

    let uuid = db.get_history().unwrap().add_entry(filtered_image);
    println!("Image {} filtered with 'median', kernel size: {}, psnr: {}!", uuid.to_string(), args.kernel_size, psnr);
}

fn low_pass(args: LowPassArgs){
    let mut db = DATABASE.lock().unwrap();
    let image = &db.get_history().unwrap().entries.last().unwrap().image;
    let (width, height) = image.dimensions();

    // Creating the FFT Image
    let mut img_buffer: Vec<Complex<f64>> = image
        .clone()
        .into_luma8()
        .as_raw()
        .iter()
        .map(|&pix| Complex::new(pix as f64 / 255.0, 0.0))
        .collect();

    fft_2d(width as usize, height as usize, &mut img_buffer);
    img_buffer = fftshift(height as usize, width as usize, &img_buffer);
    //

    // Apply the Lowpass Filter
    let low_pass = low_pass_filter(height as usize, width as usize, args.radius, args.radius);
    let fft_low_pass: Vec<Complex<f64>> = low_pass
        .iter()
        .zip(&img_buffer)
        .map(|(l, b)| l*b)
        .collect();
    //

    // Creating the Inverse of the FF after lowpass
    img_buffer = ifftshift(height as usize, width as usize, &fft_low_pass);
    ifft_2d(height as usize, width as usize, &mut img_buffer);
    let fft_coef = 1.0 / (width * height) as f64;
    for x in img_buffer.iter_mut() {
        *x *= fft_coef;
    }

    // Convert the complex img_buffer back into a gray image.
    let img_raw: Vec<u8> = img_buffer
        .iter()
        .map(|c| (c.norm().min(1.0) * 255.0) as u8)
        .collect();
    //

    let fft_view = DynamicImage::ImageLuma8(image::GrayImage::from_raw(width, height, img_raw).unwrap()); 
    let uuid = db.get_history().unwrap().add_entry( DynamicImage::ImageRgb8(fft_view.into_rgb8()));
    println!("Image {} filtered with 'low pass', radious: {}!", uuid.to_string(), args.radius);
}

fn low_pass_smooth(args: LowPassSmoothArgs){
    let mut db = DATABASE.lock().unwrap();
    let image = &db.get_history().unwrap().entries.last().unwrap().image;
    let (width, height) = image.dimensions();

    // Creating the FFT Image
    let mut img_buffer: Vec<Complex<f64>> = image
        .clone()
        .into_luma8()
        .as_raw()
        .iter()
        .map(|&pix| Complex::new(pix as f64 / 255.0, 0.0))
        .collect();

    fft_2d(width as usize, height as usize, &mut img_buffer);
    img_buffer = fftshift(height as usize, width as usize, &img_buffer);
    //

    // Apply the Lowpass Filter
    let low_pass = low_pass_filter(height as usize, width as usize, args.inner, args.outer);
    let fft_low_pass: Vec<Complex<f64>> = low_pass
        .iter()
        .zip(&img_buffer)
        .map(|(l, b)| l*b)
        .collect();
    //

    // Creating the Inverse of the FF after lowpass
    img_buffer = ifftshift(height as usize, width as usize, &fft_low_pass);
    ifft_2d(height as usize, width as usize, &mut img_buffer);
    let fft_coef = 1.0 / (width * height) as f64;
    for x in img_buffer.iter_mut() {
        *x *= fft_coef;
    }

    // Convert the complex img_buffer back into a gray image.
    let img_raw: Vec<u8> = img_buffer
        .iter()
        .map(|c| (c.norm().min(1.0) * 255.0) as u8)
        .collect();
    //

    let fft_view = DynamicImage::ImageLuma8(image::GrayImage::from_raw(width, height, img_raw).unwrap()); 
    let uuid = db.get_history().unwrap().add_entry( DynamicImage::ImageRgb8(fft_view.into_rgb8()));
    println!("Image {} filtered with 'low pass smoothed', inner radious: {}, outer radious: {}!", uuid.to_string(), args.inner, args.outer);
}


// Helpers #####################################################################

/// Convert the norm of the (transposed) FFT 2d transform into an image for visualization.
/// Use a logarithm scale.
fn view_fft_norm(width: u32, height: u32, img_buffer: &[Complex<f64>]) -> image::GrayImage {
    let fft_log_norm: Vec<f64> = img_buffer.iter().map(|c| (1f64+ c.norm()).log(10f64)).collect();
    let max_norm = fft_log_norm.iter().cloned().fold(0.0, f64::max);
    let fft_norm_u8: Vec<u8> = fft_log_norm
        .into_iter()
        .map(|x| ((x / max_norm) * 255.0) as u8)
        .collect();
    image::GrayImage::from_raw(width, height, fft_norm_u8).unwrap()
}

/// Apply a low-pass filter (6% radius, smoothed on 2%).
fn low_pass_filter(width: usize, height: usize, inner: f64, outer: f64) -> Vec<f64> {
    let diagonal = ((width * width + height * height) as f64).sqrt();
    let radius_in_sqr = (inner * diagonal).powi(2);
    let radius_out_sqr = (outer * diagonal).powi(2);
    let center_x = (width - 1) as f64 / 2.0;
    let center_y = (height - 1) as f64 / 2.0;
    let mut buffer = vec![0.0; width * height];
    for (i, row) in buffer.chunks_exact_mut(width).enumerate() {
        for (j, pix) in row.iter_mut().enumerate() {
            let dist_sqr = (center_x - j as f64).powi(2) + (center_y - i as f64).powi(2);
            *pix = if dist_sqr < radius_in_sqr {
                1.0
            } else if dist_sqr > radius_out_sqr {
                0.0
            } else {
                ((radius_out_sqr - dist_sqr) / (radius_out_sqr - radius_in_sqr)).powi(2)
            }
        }
    }
    buffer
}