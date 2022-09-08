use chrono::Local;
use std::thread;
use std::time;
use std::fs;
use rascam;
use std::io::Write;


fn main() {
	const DELAY: std::time::Duration = time::Duration::from_secs(3);
	const PATH: &str = "images";	
	const FILE_FORMAT: &str = "jpg";

	// get current date
	let date: chrono::Date<Local> = chrono::offset::Local::now().date();
	// the current date is the photo folder's name
	let dir_name: String = format!("{}/{}", PATH, date.format("%d_%m_%y"));
	// create the photo folder
	fs::create_dir_all(dir_name).unwrap();

	// get information about all installed cameras
    let info = rascam::info().unwrap();

    // check if any cameras are installed
    if info.cameras.len() < 1 {
        println!("Found 0 cameras. Exiting");
        ::std::process::exit(1);
    }

    // initiate the camera
    let mut camera = rascam::SimpleCamera::new(info.clone()).unwrap();
    camera.activate().unwrap();

	loop {
		// get current time
		let time: chrono::DateTime<Local> = chrono::offset::Local::now();

		// the time is image's file name
		let image_path = format!("{}/{}.{}", dir_name, time.format("%H_%M_%S"), FILE_FORMAT);

		// take photo
		let photo = camera.take_one().unwrap();

		// create and save the image file
    	fs::File::create(image_path).unwrap().write_all(&photo).unwrap();

		println!("image '{}' saved", image_path);

		// wait a bit
		thread::sleep(DELAY);
	}

}
