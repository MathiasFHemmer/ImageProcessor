use image::{RgbImage, GenericImageView, Rgb, Pixel, DynamicImage};
use rand::{thread_rng, Rng};

use crate::{commands::image::noise::*, DATABASE, util};

pub fn handle(args: NoiseArgs){
    match args.commands{
        NoiseCommands::SaltAndPepper(args) => salt_and_pepper(args),
    }
}

fn salt_and_pepper(args: SaltAndPepperArgs){
    let mut db = DATABASE.lock().unwrap();

    let image = &db.get_history().unwrap().entries.last().unwrap().image;
    let dimension = image.dimensions();
    let mut noise_image = RgbImage::new(dimension.0, dimension.1);

    let mut rng = thread_rng();
    for y in 0..dimension.1{
        for x in 0..dimension.0{
            if rng.gen_bool(1.0 / args.frequency as f64){
                noise_image.put_pixel(x, y, Rgb([255, 255, 255]));
            } else if rng.gen_bool(1.0 / args.frequency as f64){
                noise_image.put_pixel(x, y, Rgb([0, 0, 0]));
            } else {
                noise_image.put_pixel(x, y, image.get_pixel(x, y).clone().to_rgb());
            }
        }
    }
    
    let noise_image = DynamicImage::ImageRgb8(noise_image);

    let first_image = &db.get_history().unwrap().entries.first().unwrap().image;
    let psnr = util::psnr(first_image, &noise_image);

    let uuid = db.get_history().unwrap().add_entry(noise_image);
    println!("Image {} noised with salt and pepper, frequency: {}! Psrn: {}", uuid.to_string(), args.frequency, psnr);
}