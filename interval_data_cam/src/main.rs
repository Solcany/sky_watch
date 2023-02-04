use std::time::Duration;
use std::thread::sleep;
use std::path::Path;
use std::thread;
use std::fs::{create_dir_all, OpenOptions, write};
use gphoto2::{Context, Camera, Result};
use image::{DynamicImage, ImageFormat, load_from_memory_with_format, imageops};
use turbojpeg;
use reqwest::blocking::get;
use serde_json::Value;
use serde::Serialize;
use csv::Writer;
use chrono::{Local, Datelike, Timelike};

const OUTPUT_PATH : &str = "./output/sessions";
const IMAGES_FOLDER : &str = "images";
const IMAGE_EXT : &str = ".jpg";
const CSV_FOLDER : &str = "csv";
const DELAY_SECONDS : u32 = 4;
const IMAGE_SCALAR : f32 = 0.5;

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

fn process_save_image(image: DynamicImage, path : String) -> Result<()> {
    let new_width : u32 = (image.width() as f32 * IMAGE_SCALAR).floor() as u32;
    let new_height : u32 = (image.height() as f32 * IMAGE_SCALAR).floor() as u32;
    let new_image = imageops::resize(&image, 
                                     new_width, 
                                     new_height, 
                                     imageops::FilterType::Gaussian); 

    let data = turbojpeg::compress_image(&new_image, 70, turbojpeg::Subsamp::Sub2x2).unwrap();

    write(&path, &data).unwrap();
    //data.save_with_format(path, image::ImageFormat::Jpeg).unwrap();
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
            let output_path : String = format!("{}/{}/{}/{}.{}", OUTPUT_PATH, &session_name, IMAGES_FOLDER, &image_name, IMAGE_EXT);

            match process_save_image(image, output_path) {
                Ok(()) => {
                    println!("image: {} saved", &image_name);
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

  // let captured_file_path = camera
  //   .capture_image()
  //   .wait()?;

  // let captured_file = camera
  //   .fs()
  //   .download(&captured_file_path.folder(), &captured_file_path.name())
  //   .wait()
  //   .unwrap();

  // let photo_name = captured_file.name();
  // let photo_bytes = captured_file
  //   .get_data(&context)
  //   .wait()?;  

  // match image::load_from_memory_with_format(&photo_bytes, image::ImageFormat::Jpeg) {
  //   Ok(image) => {
  //       //write("output.jpg", &bytes).unwrap();
  //   }
  //   Err(_) => {
  //       println!("input is not png");
  //   }
  // }


    Ok(())
} 