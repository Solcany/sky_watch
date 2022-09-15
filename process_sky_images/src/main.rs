use image;
use csv::Writer;
use std::fs;
// use std::error::Error;


// fn get_arr_size(w: &u32, h: &u32) -> usize {
// 	let size = w * h;
// 	size as usize
// }

// fn add_record_to_writer(writer: &Writer, arr: &[String]) {
//     //let mut wtr = Writer::from_path("image_data.csv")?;
//     writer.write_record(&arr)?;
//     //wtr.flush()?;
//     //Ok(())
// }


fn main() {
	const IN_PATH : &str = "./images/friday/low_res";
	const OUT_PATH : &str = "./data";
	const CROP_X: u32 = 0;
	const CROP_Y: u32 = 0;
	const CROP_WIDTH : u32 = 800;
	const CROP_HEIGHT : u32 = 800;
	const SAMPLE_WIDTH: u32 = 10;
	const SAMPLE_HEIGHT: u32 = 10;
	const SAMPLE_SIZE: usize = 100;


	let out_csv_path : String = format!("{}/{}", OUT_PATH, "data.csv");

	// make the writer mutable so rows can be written into it
	let mut writer = Writer::from_path(out_csv_path).unwrap();

	// make the paths mutable so they can be sorted
    let mut image_paths : Vec<fs::DirEntry> = fs::read_dir(IN_PATH).unwrap()
    														   	   .map(|f| f.unwrap())
    														       .collect();
    image_paths.sort_by_key(|f| f.path());    

    for path in image_paths {

    	println!("working on image: {}", &path.path().display());

		// load image
		let img = image::open(&path.path()).unwrap();

		// grayscale fn returns ImageBuffer
		let grayscale_img = image::imageops::colorops::grayscale(&img);

		// crop_imm returns immutable view into the image (SubImage type)
		// .to_image fn converts the view into an ImageBuffer
		let cropped_img = image::imageops::crop_imm(&grayscale_img, 
													CROP_X, 
													CROP_Y, 
													CROP_WIDTH, 
													CROP_HEIGHT).to_image();

		// borrows ImageBuffer and returns a new ImageBuffer
		let rezized_img = image::imageops::resize(&cropped_img, 
												  SAMPLE_WIDTH, 
												  SAMPLE_HEIGHT, 
												  image::imageops::FilterType::Gaussian);

		// initiate empty array of SAMPLE_SIZE filled with zeroes
		let mut pixels :[f32; SAMPLE_SIZE] = [0.0; SAMPLE_SIZE];
		// normalise 0 – 255 pixel values to 0.0 – 1.0 range
		for (i, pixel) in rezized_img.pixels() 
									 .enumerate() { // get indices
			pixels[i] = pixel.0[0] as f32 / 255.0;  // cast integer to float
		}

		// conver the float values to strings
		// note: the new 'values' var declaration shadows the previous one
	    let pixels : Vec<String> = pixels.into_iter() // array to iterator
	    								 .map(|v| v.to_string()) // convert float to string
	    								 .collect(); // iterator to collection
	    // write a new row to the csv
	    writer.write_record(&pixels).unwrap();
	};

	writer.flush().unwrap();
	println!("{}", "done!");

	//cropped_img.save(OUT_PATH).unwrap();
}