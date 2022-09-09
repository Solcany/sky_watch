use chrono;
use rascam;
use std::fs;
use std::io::Write;


fn main() {
	const PATH: &str = "./images";	
	const FILE_FORMAT: &str = "jpg";

	// get information about all installed cameras
    let info = rascam::info().unwrap();

    // check if any cameras are installed
    if info.cameras.len() < 1 {
        println!("Found 0 cameras. Exiting");
        ::std::process::exit(1);
    }

    // select camera
    let active_cam = info.cameras[0].clone();

    // initiate the camera
    let mut camera = rascam::SimpleCamera::new(active_cam).unwrap();
    camera.activate().unwrap();

	// get current time
	let time: chrono::DateTime<chrono::Local> = chrono::offset::Local::now();

	// the time is image's file name
	let image_path = format!("{}/{}.{}", PATH, time.format("%H_%M_%S_%m_%d"), FILE_FORMAT);

	// take photo
	let photo = camera.take_one().unwrap();

	// create and save the image file
	fs::File::create(&image_path).unwrap().write_all(&photo).unwrap();

	println!("image '{}' saved", &image_path);

}
