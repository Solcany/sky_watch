use std::time::Duration;
use std::thread::sleep;
use std::path::Path;
use std::thread;
use std::fs::{create_dir_all, OpenOptions, write};
use gphoto2::{Context, Camera, Result};
use image::{DynamicImage, ImageBuffer, ImageFormat, Rgba, load_from_memory_with_format, imageops};
use turbojpeg;
use reqwest::blocking::get;
use serde_json::Value;
use serde::Serialize;
use csv::Writer;
use chrono::{Local, Datelike, Timelike};

const OUTPUT_PATH : &str = "./output/sessions";
const IMAGES_FOLDER : &str = "images";
const CSV_FOLDER : &str = "csv";
const DELAY_SECONDS : u32 = 4;
const IMAGE_SCALAR : f32 = 0.5;
const JPG_COMPRESSION : i32 = 70;

const DATA_URL : &str = "https://data.buienradar.nl/2.0/feed/json";
const OUT_PATH : &str = "./";
const CSV_NAME : &str = "data.csv";



fn get_session_name() -> String {
    let current_datetime = Local::now();
    let day = current_datetime.day0(); 
    let month = current_datetime.month0();    
    let hour = current_datetime.hour();        
    let minute = current_datetime.minute();    
    let year = current_datetime.year();
    format!("{}_{}_{}_{}_{}", &day, &month, &hour, &minute, &year)
}

fn create_session_output_dir(session_name : &String) {
    let images_path : String = format!("{}/{}/{}", OUTPUT_PATH, &session_name, IMAGES_FOLDER);
    let csv_path : String = format!("{}/{}/{}", OUTPUT_PATH, &session_name, CSV_FOLDER);
    // create all export dirs if they don't exist
    create_dir_all(&images_path).unwrap();    
    create_dir_all(&csv_path).unwrap();    
}

fn capture_photo(camera_context : &Context, camera : &Camera) -> Result<(DynamicImage, String)> {
    let captured_file_path = camera
        .capture_image()
        .wait()?;
    let photo_folder = &captured_file_path
        .folder();
    let photo_name = &captured_file_path
        .name();
    let captured_file = camera
        .fs()
        .download(&photo_folder, &photo_name)
        .wait()?;
    let photo_bytes = captured_file
        .get_data(&camera_context)
        .wait()?; 
    let photo_image = load_from_memory_with_format(&photo_bytes, ImageFormat::Jpeg)
        .unwrap();
    Ok((photo_image, photo_name.to_string()))
}
fn process_image(image: DynamicImage) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>> {
    let new_width : u32 = (image.width() as f32 * IMAGE_SCALAR).floor() as u32;
    let new_height : u32 = (image.height() as f32 * IMAGE_SCALAR).floor() as u32;
    let new_image = imageops::resize(&image, 
                                     new_width, 
                                     new_height, 
                                     imageops::FilterType::Gaussian); 
    Ok(new_image)
}
fn save_image(image : ImageBuffer<Rgba<u8>, Vec<u8>>, path: String) -> Result<()> {
    let jpg_data = turbojpeg::compress_image(&image, JPG_COMPRESSION, turbojpeg::Subsamp::Sub2x2).unwrap();
    write(&path, &jpg_data).unwrap();
    Ok(())
}
    
fn main() -> Result<()> {
    let session_name : String = get_session_name();
    create_session_output_dir(&session_name);

    //initiate the camera
    let camera_context = Context::new()?;
    let camera = camera_context.autodetect_camera().wait()?;

    // take a photo
    match capture_photo(&camera_context, &camera) {
        Ok(image_data) => {
            println!("image captured successfully");
            let (image, image_name) = image_data;
            let processed_image = process_image(image).unwrap();
            let output_path : String = format!("{}/{}/{}/{}", OUTPUT_PATH, &session_name, IMAGES_FOLDER, &image_name);
            match save_image(processed_image, output_path) {
                Ok(processed_image) => {
                    println!("image: {} saved", &output_path);
                }
                Err(err) => {
                    println!("failed to save the image");
                }
            }
        }
        Err(err) => {
            println!("failed to capture image");
        }
      }    
    Ok(())
} 