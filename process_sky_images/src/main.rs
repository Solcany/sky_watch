use image;
use csv::Writer;
use std::fs;


fn rerange_float(value: f32, 
				 min1: f32,
				 max1: f32,
				 min2: f32,
				 max2: f32 ) -> f32 {
	// translate a float value to a new range from old
	return ((value - min1) * (max2 - min2)) / (max1 - min1) + min2;
}

fn main() {
	const IN_PATH : &str = "./images/friday/low_res";
	const OUT_PATH : &str = "./data";
	const IMAGE_SAMPLES_OUT_PATH : &str = "./data/downsized";
	const CROP_X: u32 = 0; // where does the crop start?
	const CROP_Y: u32 = 0; 
	const CROP_WIDTH : u32 = 800; // how wide is the crop of the original image?
	const CROP_HEIGHT : u32 = 800;
	const SAMPLE_WIDTH: u32 = 16; // to what width should the image be down sized?
	const SAMPLE_HEIGHT: u32 = 16;
	const SAMPLE_SIZE: usize = 256; // how many pixels are in the sample? ( basically SAMPLE_WIDTH * SAMPLE_HEIGHT)
	const C_MIN : f32 = 0.0; // low limit of the color range of image
	const C_MAX : f32 = 255.0; // high limit of the color range of image
	const NEW_MIN : f32 = 0.0; // low limit of the new range
	const NEW_MAX : f32 = 1.0; // high limit of the new range

	// create all export dirs if they don't exist
	std::fs::create_dir_all(OUT_PATH).unwrap();
	std::fs::create_dir_all(IMAGE_SAMPLES_OUT_PATH).unwrap();

	// set path for the csv export file
	let out_csv_path : String = format!("{}/{}", OUT_PATH, "data.csv");

	// make the writer mutable so rows can be written into it
	let mut writer = Writer::from_path(out_csv_path).unwrap();

	// make the paths mutable so they can be sorted
    let mut image_paths : Vec<fs::DirEntry> = fs::read_dir(IN_PATH).unwrap()
    														   	   .map(|f| f.unwrap())
    														       .collect();
    // sort the paths alphabetically														       
    image_paths.sort_by_key(|f| f.path());    

    
    for path in image_paths {

    	// show processing progress 
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

		// note: borrows ImageBuffer and returns a new ImageBuffer
		let rezized_img = image::imageops::resize(&cropped_img, 
												  SAMPLE_WIDTH, 
												  SAMPLE_HEIGHT, 
												  image::imageops::FilterType::Gaussian);

		// save the processed image sample to a file
		let sample_image_out_path = format!("{}/{:?}", IMAGE_SAMPLES_OUT_PATH, &path.file_name());
		rezized_img.save_with_format(sample_image_out_path, image::ImageFormat::Jpeg).unwrap();

		// initiate empty array of SAMPLE_SIZE filled with zeroes
		// note: this could be a vector instead, the initialisation is a bit clunky with Array
		let mut pixels :[f32; SAMPLE_SIZE] = [0.0; SAMPLE_SIZE];
		// normalise 0 – 255 pixel values to 0.0 – 1.0 range
		for (i, pixel) in rezized_img.pixels() // get pixels iterator
									 .enumerate() { // get indices
			// convert the color gray value of 0..255 range to a NEW_MIN, NEW_MAX range
			pixels[i] = rerange_float(pixel.0[0] as f32, C_MIN, C_MAX, NEW_MIN, NEW_MAX);
		}

		// convert the float values to strings
		// note: the new 'values' var declaration shadows the previous one
	    let pixels : Vec<String> = pixels.into_iter() // array to iterator
	    								 .map(|v| v.to_string()) // convert float to string
	    								 .collect(); // iterator to collection
	    // write a new row to the csv
	    writer.write_record(&pixels).unwrap();
	};

	writer.flush().unwrap();
	println!("{}", "done!");

}
