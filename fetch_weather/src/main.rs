use reqwest;
use serde_json;


fn get_eindhoven_station_data(weather_json : serde_json::Value) -> Option<serde_json::Value>{

    let weather_stations_data = weather_json["actual"]["stationmeasurements"]
        .as_array()
        .unwrap();

    let mut eindhoven_data : Option<serde_json::Value> = None;

    for station_data in weather_stations_data.iter() {
        if station_data["stationname"] == "Meetstation Eindhoven" {
            eindhoven_data = Some(station_data.clone());
        }
    }

    return eindhoven_data
}


fn main() {
    let weather_json = reqwest::blocking::get("https://data.buienradar.nl/2.0/feed/json")
    .unwrap()
    .json::<serde_json::Value>()
    .unwrap();

    let eindhoven_weather_data : serde_json::Value = get_eindhoven_station_data(weather_json).unwrap();



}