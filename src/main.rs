use chrono::Local;
use std::thread;
use std::time;
use std::fs;

fn main() {
	const DELAY: std::time::Duration = time::Duration::from_secs(2);
	const PATH: &str = "images";	
	const FILE_FORMAT: &str = "jpg";

	// create the folder for the sky photos
	let date: chrono::Date<Local> = chrono::offset::Local::now().date();
	// the day's date is the folder name
	let dir_name: String = format!("{}/{}", PATH, date.format("%d_%m_%y"));

	fs::create_dir_all(dir_name).unwrap();

	loop {
		let time: chrono::DateTime<Local> = chrono::offset::Local::now();
		let out_path = format!("{}/{}.{}", PATH, time.format("%H_%M_%S"), FILE_FORMAT);
		println!("time: {}", out_path);

		thread::sleep(DELAY);
	}

}
