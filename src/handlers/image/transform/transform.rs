use fft2d::slice::fft_2d;
use fft2d::slice::fftshift;
use image::DynamicImage;
use image::GenericImageView;
use image::GrayImage;
use rustfft::num_complex::Complex;

use crate::DATABASE;
use crate::commands::image::transform::TransformArgs;
use crate::commands::image::transform::TransformCommands;

pub fn handle(args: TransformArgs){
    match args.commands {
        TransformCommands::Fourier => fourier_transform(),
    }
}

fn fourier_transform(){
    let mut db = DATABASE.lock().unwrap();
    let image = &db.get_history().unwrap().get_entry().unwrap().image;

    let (width, height) = image.dimensions();

    let mut img_buffer: Vec<Complex<f64>> = image
        .clone()
        .into_luma8()
        .as_raw()
        .iter()
        .map(|&pix| Complex::new(pix as f64 / 255.0, 0.0))
        .collect();

    fft_2d(width as usize, height as usize, &mut img_buffer);

    img_buffer = fftshift(height as usize, width as usize, &img_buffer);

    let fft_view = DynamicImage::ImageLuma8(view_fft_norm(height, width, &img_buffer)); 

    let uuid = db.get_history().unwrap().add_entry( DynamicImage::ImageRgb8(fft_view.into_rgb8()));
    println!("Fourier image {} created!", uuid.to_string());
}


// Helpers #######################################################

fn view_fft_norm(width: u32, height: u32, img_buffer: &[Complex<f64>]) -> image::GrayImage {
    let fft_log_norm: Vec<f64> = img_buffer.iter().map(|c| (c.norm()).log(10f64)).collect();
    let max_norm = fft_log_norm.iter().cloned().fold(0.0, f64::max);
    let fft_norm_u8: Vec<u8> = fft_log_norm
        .into_iter()
        .map(|x| ((x / max_norm) * 255.0) as u8)
        .collect();
    
    GrayImage::from_raw(width, height, fft_norm_u8).unwrap()
}