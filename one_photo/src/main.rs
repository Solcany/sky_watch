use chrono::Local;
use std::thread;
use std::time;
use std::fs;
use rascam;
use std::io::Write;

fn main() {
	const DELAY: std::time::Duration = time::Duration::from_millis(10);	
	const PATH: &str = "images";	
	const FILE_FORMAT: &str = "jpg";
	const COMMAND_START: &str = "scp m@192.168.1.77:~/Documents/rust/sky_watch/one_photo/";
	const COMMAND_END:  &str = " ~/Desktop";

    let settings = rascam::CameraSettings {
        encoding: rascam::MMAL_ENCODING_RGB24,
        width: 0,
        height: 0,
        iso: rascam::ISO_AUTO,
        zero_copy: true,
        use_encoder: false,
    };

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
    camera.set_camera_format(&settings).unwrap();
    
    camera.activate().unwrap();

	// get current time
	let time: chrono::DateTime<chrono::Local> = chrono::offset::Local::now();

	// the time is image's file name
	let image_path = format!("{}/{}.{}", PATH, time.format("%H_%M_%S_%m_%d"), FILE_FORMAT);

//	thread::sleep(DELAY);

	// take photo
	let photo = camera.take_one().unwrap();

//	thread::sleep(DELAY);	

	// create and save the image file
	fs::File::create(&image_path).unwrap().write_all(&photo).unwrap();

	println!("image '{}' saved", &image_path);
	println!("{}{}/{}", COMMAND_START, &image_path, COMMAND_END);

}
