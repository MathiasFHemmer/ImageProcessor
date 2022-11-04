use std::collections::HashMap;

use image::{imageops::grayscale_alpha, DynamicImage, GenericImageView};
use plotters::prelude::*;
use show_image::{create_window, event};

use crate::{commands::image::image::{ImageArgs, ImageCommands, AddArgs, CheckoutArgs, PSNRArgs}, DATABASE, util};

use super::{filter, noise, transform};

pub fn handle(args: ImageArgs){
    match args.commands{
        ImageCommands::Add(args) => add(args),
        ImageCommands::Show => show(),
        ImageCommands::Filter(cmd) => filter::handle(cmd),
        ImageCommands::Noise(cmd) => noise::handle(cmd),
        ImageCommands::Checkout(args) => checkout(args),
        ImageCommands::List => list(),
        ImageCommands::Clone => clone(),
        ImageCommands::Grayscale => grayscale(),
        ImageCommands::Transform(cmd) => transform::handle(cmd),
        ImageCommands::Undo => undo(),
        ImageCommands::Histogram => histogram(),
        ImageCommands::PSNR(args) => psnr(args),
    }
}

fn undo(){
    let mut db = DATABASE.lock().unwrap();
    let history = db.get_history().unwrap();

    history.undo();
    
    history.selected_image_uuid = Some(history.entries.last().unwrap().uuid);

    println!("Last action was undone!");
}

fn list() {
    let mut db = DATABASE.lock().unwrap();
    let history = db.get_history().unwrap();

    for (index, entry) in history.entries.iter_mut().enumerate(){
        println!("Identifier: {}. Index: {}", entry.uuid, index);
    }
}

pub fn add(args : AddArgs){
    let image = image::open(args.path.clone());
    if image.is_err(){
        println!("Error opening image {}: {}!", args.path, image.err().unwrap().to_string());
        return;
    }
    
    let mut db = DATABASE.lock().unwrap();
    let new_history_id = db.new_entry(image.unwrap());

    println!("New image added at {} from {}", new_history_id, args.path);
    if new_history_id == 1{
        db.selected_history_id = new_history_id;
        println!("First repository created. Automatic checkout executed!");
    }
}

pub fn clone(){
    let mut db = DATABASE.lock().unwrap();
    let history = db.get_history().unwrap();
    let entry_uuid = history.get_entry().unwrap().uuid;
    let image = history.get_entry().unwrap().image.clone();

    let new_history_id = db.new_entry(image);

    println!("Image {} cloned to new repository at: {}", entry_uuid.to_string(), new_history_id);
}

pub fn show(){
    let mut db = DATABASE.lock().unwrap();
    let history = db.get_history().unwrap();
    let image = history.get_entry().unwrap().image.clone();

    let window = create_window("Imagem Original", Default::default()).unwrap();
    window.set_image("v1", image).unwrap();

    for event in window.event_channel().unwrap() {
        if let event::WindowEvent::KeyboardInput(event) = event {
            if event.input.key_code == Some(event::VirtualKeyCode::Escape) && event.input.state.is_pressed() {
                break;
            }
        }
    }
    ();
}

pub fn checkout(args: CheckoutArgs){
    let mut db = DATABASE.lock().unwrap();

    let status = db.get_history().unwrap().change_selected(&args.id);

    match status {
        true => println!("Checked out image {}!", &args.id),
        false => println!("No image with id {}!", &args.id),
    }
}

fn grayscale(){
    let mut db = DATABASE.lock().unwrap();

    let image = &db.get_history().unwrap().entries.last().unwrap().image;
    let grayscale_image = DynamicImage::ImageRgb8(image.grayscale().to_rgb8());

    let first_image = &db.get_history().unwrap().entries.first().unwrap().image;
    let psnr = util::psnr(first_image, &grayscale_image);

    let uuid = db.get_history().unwrap().add_entry(grayscale_image);
    println!("Image grayscaled at {}! Psrn: {}", uuid.to_string(), psnr);
}

fn histogram(){
    let mut db = DATABASE.lock().unwrap();
    let image = &db.get_history().unwrap().entries.last().unwrap().image;

    let root = BitMapBackend::new("__tmp.png", (1280, 720)).into_drawing_area();
    let data = image.grayscale().as_luma8().unwrap().as_raw().clone();

    let max = data.iter()
    .fold(HashMap::<u8, usize>::new(), |mut m, x| {
        *m.entry(*x).or_default() += 1;
        m
    })
    .into_iter()
    .max_by_key(|(_, v)| *v)
    .map(|(_, v)| v)
    .unwrap();

    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(25)
        .y_label_area_size(25)
        .margin(5)
        .caption("Histograma", ("sans-serif", 50.0))
        .build_cartesian_2d((0u32..255u32).into_segmented(), 0u32..((max + max/10) as u32)).unwrap();

    chart
        .configure_mesh()
        .disable_x_mesh()
        .bold_line_style(&WHITE.mix(0.3))
        .y_desc("Count")
        .x_desc("Bucket")
        .axis_desc_style(("sans-serif", 15))
        .draw().unwrap();

    chart.draw_series(
        Histogram::vertical(&chart)
            .style(RED.mix(0.2).filled())
            .data(data.iter().map(|x| (*x as u32, 1))),
    ).unwrap();

    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    let image = image::open("__tmp.png").unwrap();
    std::fs::remove_file("__tmp.png").unwrap();

    let uuid = db.get_history().unwrap().add_entry(image);
    println!("Grayscale Histogram {} generated!", uuid.to_string());
}

fn psnr(args: PSNRArgs){
    let mut db = DATABASE.lock().unwrap();
    let image = &db.get_history().unwrap().get_entry().unwrap().image;

    let copare = image::open(args.path.clone());
    if copare.is_err(){
        println!("Error opening image {}: {}!", args.path, copare.err().unwrap().to_string());
        return;
    }
    let psnr = util::psnr(image, &copare.unwrap());

    println!("Psrn: {} generated from image {} comparasion", psnr, args.path);
}