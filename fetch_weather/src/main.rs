use reqwest::blocking::get;
use serde_json::Value;
use serde::Serialize;
use std::path::Path;
use csv::Writer;
use std::fs::OpenOptions;
use std::time::Duration;
use std::thread::sleep;
use chrono::{Local, Datelike, Timelike};
const FETCH_DELAY_SECONDS : u32 = 4;
const DATA_URL : &str = "https://data.buienradar.nl/2.0/feed/json";
const OUT_PATH : &str = "./";
const CSV_NAME : &str = "data.csv";

#[derive(Serialize)]
struct Row<'a> {
    photo_name: &'a str,
    photo_timestamp: &'a str,
    air_pressure: f64,
    temperature: f64,
    feel_temperature: f64,    
    weather_description: &'a str,
    weather_timestamp: &'a str,    
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
    return eindhoven_data
}


fn main() {
    let csv_path : String = format!("{}/{}", OUT_PATH, CSV_NAME);    
    let mut writer;
    // initialise the csv writer
    if Path::new(&csv_path).exists() {
        // if there's existing csv on the path append new rows to it
        let csv_file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(csv_path)
            .unwrap();
        writer = Writer::from_writer(csv_file);
    } else {
        // otherwise create new csv file 
        writer = Writer::from_path(csv_path).unwrap();
    }

    let fetch_delay = Duration::from_secs(FETCH_DELAY_SECONDS as u64);
    let mut c : u32 = 0;

    loop {
        println!("row : {}", c);
        if c == 5 {
            println!("done!");
            break;
        }

        let current_datetime = Local::now();
        let year = current_datetime.year();
        let month = current_datetime.month0();
        let day = current_datetime.day0(); 
        let hour = current_datetime.hour();        
        let minute = current_datetime.minute();
        let second = current_datetime.second();
        let timezone : &str = "+01:00";

        //let timestamp: i64 = dt.timestamp();

        println!("{}-{}-{}-{}:{}:{}-{}", year, month, day, hour, minute, second, timezone);

        // fetch data from the api
        let eindhoven_weather_data = fetch_eindhoven_weather_data();

        match eindhoven_weather_data {
            Some(data) => {
                // write new row
                let row : Row = Row {  
                                   photo_name: "name",
                                   photo_timestamp: "heh",
                                   air_pressure: data["airpressure"].as_f64().unwrap(),
                                   temperature: data["temperature"].as_f64().unwrap(),
                                   feel_temperature: data["feeltemperature"].as_f64().unwrap(),
                                   weather_description: data["weatherdescription"].as_str().unwrap(),
                                   weather_timestamp: data["timestamp"].as_str().unwrap(),
                                };
                writer.serialize(row).unwrap();
                writer.flush().unwrap();
            },
            None => println!("{}", "Eindhoven data not foundß"),
        }

        sleep(fetch_delay);
        c += 1;        
    }

}