use std::time::Duration;
use std::thread::sleep;
use std::path::Path;
use std::fs::{OpenOptions, create_dir_all, write};
use gphoto2::{Context, Camera, Result};
use image::{DynamicImage, ImageBuffer, ImageFormat, Rgba, load_from_memory_with_format, imageops};
use turbojpeg;
use reqwest::blocking::get;
use serde_json::Value;
use serde::Serialize;
use csv::Writer;
use chrono::{Local, Datelike, Timelike};
use std::process::abort;

const OUTPUT_PATH : &str = "./output/sessions";
const IMAGES_FOLDER : &str = "images";
const CSV_FOLDER : &str = "csv";
const DELAY_SECONDS : u32 = 4;
const IMAGE_SCALAR : f32 = 0.5;
const JPG_COMPRESSION : i32 = 70;
const CAPTURE_DELAY_SECONDS : u32 = 240;
const DATA_URL : &str = "https://data.buienradar.nl/2.0/feed/json";
const CSV_NAME : &str = "data.csv";

#[derive(Serialize)]
struct Csv_row<> {
    photo_name: String,
    photo_timestamp: String,
    air_pressure: f64,
    temperature: f64,
    feel_temperature: f64,
    ground_temperature: f64,
    visibility: f64,
    wind_gusts: f64,
    wind_speed: f64,
    wind_direction: String,
    wind_direction_degrees: f64,
    humidity: f64,
    precipitation: f64,
    sunpower: f64,
    rainfall_last_24_hours: f64,
    rainfall_last_hour: f64,
    weather_description: String,                          
    weather_timestamp: String,  
}

fn get_session_name() -> String {
    let current_datetime = Local::now();
    let day = current_datetime.day(); 
    let month = current_datetime.month();    
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
    // rescale the image
    let new_width : u32 = (image.width() as f32 * IMAGE_SCALAR).floor() as u32;
    let new_height : u32 = (image.height() as f32 * IMAGE_SCALAR).floor() as u32;
    let new_image = imageops::resize(&image, 
                                     new_width, 
                                     new_height, 
                                     imageops::FilterType::Gaussian); 
    Ok(new_image)
}

fn save_image(image : ImageBuffer<Rgba<u8>, Vec<u8>>, path: &String) -> Result<()> {
    // compress the jpg image
    let jpg_data = turbojpeg::compress_image(&image, JPG_COMPRESSION, turbojpeg::Subsamp::Sub2x2).unwrap();
    write(&path, &jpg_data).unwrap();
    Ok(())
}

fn fetch_eindhoven_weather_data() -> Option<Value> {
    // fetch weather data from the buienradar api
    let weather_json = get(DATA_URL)
        .unwrap()
        .json::<serde_json::Value>()
        .unwrap();

    // get relevant data from the json
    let weather_stations_data = weather_json["actual"]["stationmeasurements"]
        .as_array()
        .unwrap();

    // initialise return variable
    let mut eindhoven_data : Option<serde_json::Value> = None;

    // find eindhoven data
    for station_data in weather_stations_data.iter() {
        if station_data["stationname"] == "Meetstation Eindhoven" {
            // create a copy
            eindhoven_data = Some(station_data.clone());
        }
    }
    eindhoven_data
}

fn get_photo_timestamp() -> String {
    let current_datetime = Local::now();
    let year = current_datetime.year();
    let month = current_datetime.month();
    let day = current_datetime.day(); 
    let hour = current_datetime.hour();        
    let minute = current_datetime.minute();
    let second = current_datetime.second();
    let timezone : &str = "+01:00";
    format!("y:{}-m:{}-d:{}-h:{}-mn:{}-s:{}-t:{}", year, month, day, hour, minute, second, timezone)
}
    
fn main() -> Result<()> { 
    // get current datetime as the session name
    let session_name : String = get_session_name();
    create_session_output_dir(&session_name);

    //initiate the camera
    let camera_context = Context::new()?;
    let camera = camera_context.autodetect_camera().wait()?;

    //initiate the csv writer
    let csv_path : String = format!("{}/{}/{}/{}", OUTPUT_PATH, &session_name, CSV_FOLDER, CSV_NAME);    
    let mut csv_writer; 
    // initialise the csv writer
    if Path::new(&csv_path).exists() {
        // if there's existing csv on the path append new rows to it
        let csv_file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(csv_path)
            .unwrap();
        csv_writer = Writer::from_writer(csv_file);
    } else {
        // otherwise create new csv file 
        csv_writer = Writer::from_path(csv_path).unwrap();
    }    
    let capture_delay = Duration::from_secs(CAPTURE_DELAY_SECONDS as u64);
    let mut photo_count : u32 = 0;

    loop {
        println!("starting capture n: {}", photo_count);

        let photo_timestamp : String = get_photo_timestamp();
        // take a photo    
        match capture_photo(&camera_context, &camera) {
            Ok(image_data) => {
                println!("image captured");
                let (image, image_name) = image_data;
                let processed_image = process_image(image).unwrap();
                // download weather data from buienradar
                // wip: weather data should be downloaded immediately after gphoto2's capture_image
                // is called. Currently weather data capture is delayed by ~ 1 minute by
                // image data being downloaded from the camera
                let data_row : Csv_row = match fetch_eindhoven_weather_data() {
                    Some(data) => {
                        Csv_row {  
                            photo_name: image_name.clone(),
                            photo_timestamp: photo_timestamp,
                            air_pressure: data["airpressure"].as_f64().unwrap(),
                            temperature: data["temperature"].as_f64().unwrap(),
                            feel_temperature: data["feeltemperature"].as_f64().unwrap(),
                            ground_temperature: data["groundtemperature"].as_f64().unwrap(),
                            visibility: data["visibility"].as_f64().unwrap(),
                            wind_gusts: data["windgusts"].as_f64().unwrap(),
                            wind_speed: data["windspeed"].as_f64().unwrap(),
                            wind_direction: data["winddirection"].as_str().unwrap().to_string(),
                            wind_direction_degrees: data["winddirectiondegrees"].as_f64().unwrap(),
                            humidity: data["humidity"].as_f64().unwrap(),
                            precipitation: data["precipitation"].as_f64().unwrap(),
                            sunpower: data["sunpower"].as_f64().unwrap(),
                            rainfall_last_24_hours: data["rainFallLast24Hour"].as_f64().unwrap(),
                            rainfall_last_hour: data["rainFallLastHour"].as_f64().unwrap(),
                            weather_description: data["weatherdescription"].as_str().unwrap().to_string(),                          
                            weather_timestamp: data["timestamp"].as_str().unwrap().to_string().to_string(),
                        }
                    },
                    None => {
                        println!("failed to fetch weather data, aborting.");
                        abort();
                    },
                };
                let image_path : String = format!("{}/{}/{}/{}", OUTPUT_PATH, &session_name, IMAGES_FOLDER, &image_name);
                match save_image(processed_image, &image_path) {
                    Ok(()) => {
                        println!("image: '{}' saved", &image_path);
                        // write csv row if image was saved successfuly
                        csv_writer.serialize(data_row).unwrap();
                        match csv_writer.flush() {
                            Ok(()) => { csv_writer.flush().unwrap();
                                        println!("csv row written"); 
                            },
                            Err(err) => println!("failed to write data row"),                        
                        }
                    },
                    Err(err) => {
                        println!("failed to save the '{}' image", &image_path);
                    }
                }
            },
            Err(err) => {
                println!("failed to capture image");
            }
          }
    photo_count += 1;
    sleep(capture_delay);
    } 
    Ok(())
} 