use image;
use csv::Writer;
use std::fs;

const IN_PATH : &str = "./images/friday/small_batch";
const CSV_OUT_PATH : &str = "./data/tile_data/csv/";
const IMAGE_OUT_PATH_ROOT: &str ="./data/tile_data/image/";
const PIXEL_MIN: f32 = 0.0;
const PIXEL_MAX: f32 = 255.0;
const NEW_PIXEL_MIN: f32 = -1.0;
const NEW_PIXEL_MAX: f32 = 1.0;
const IMAGE_WIDTH: u32 = 340;
const GRID_WIDTH_RATIO: f32 = 0.9;
const GRID_COLS: u32 = 4;
const GRID_ROWS: u32 = 4;
const EXPORT_TILE_WIDTH: u32 = 16;
const TILE_ASPECT_RATIO: f32 = 1.0;

fn lerp_range(value: &f32, min1: &f32, max1: &f32, min2: &f32, max2: &f32) -> f32{
  ((value - min1) * (max2 - min2)) / (max1 - min1) + min2
}

fn main() {
	assert!((IMAGE_WIDTH as f32 * GRID_WIDTH_RATIO) / GRID_COLS as f32 > EXPORT_TILE_WIDTH as f32, "too many tiles, decrease GRID_COLS");
	assert!(GRID_WIDTH_RATIO > 1.0, "GRID_WIDTH_RATIO must be less or equal to 1.0");
	assert!(GRID_WIDTH_RATIO <= 0.0, "GRID_WIDTH_RATIO must be more than 0.0");

	// make the paths mutable so they can be sorted
    let mut image_paths : Vec<fs::DirEntry> = fs::read_dir(IN_PATH).unwrap()
    														   	   .map(|f| f.unwrap())
    														       .collect();
    image_paths.sort_by_key(|f| f.path());

    // create a separate csv for every tile of the image
    // tile is a particular region cropped from all images in the folder 
    let mut writers = Vec::new();
    for idx in 0..GRID_COLS*GRID_ROWS {
	let out_csv_path : String = format!("{}/tile{}.csv", CSV_OUT_PATH, idx.to_string());
    	writers.push(Writer::from_path(out_csv_path).unwrap());
    }
    // get grid width based on the width of the image
    let grid_width: u32 = (IMAGE_WIDTH as f32 * GRID_WIDTH_RATIO).ceil() as u32;
    // where is the start of the first tile on x axis?
    let x_start: u32 = (IMAGE_WIDTH - grid_width) / 2;
    let tile_width : u32 = grid_width / GRID_COLS;
    let tile_height : u32 = (tile_width as f32 * TILE_ASPECT_RATIO).ceil() as u32;
    // get grid height based on the amount and height of individual tiles
    let grid_height : u32 = tile_height * GRID_ROWS;
    // where does the first tile start on the y axis?
    let y_start: u32 = (IMAGE_WIDTH - grid_height) / 2;
    // the tile will be scaled down to export tile height
    let export_tile_height: u32 = (EXPORT_TILE_WIDTH as f32 * TILE_ASPECT_RATIO).ceil() as u32;

    for (image_index, image_path) in image_paths.iter().enumerate() {
    	println!("working on image: {}", &image_path.path().display());
		// load image
		let img = image::open(&image_path.path()).unwrap();
		// grayscale fn returns ImageBuffer
		let grayscale_img = image::imageops::colorops::grayscale(&img);

		for x in 0..GRID_ROWS {
			let x_pos = x_start + (x * tile_width);
			for y in 0..GRID_COLS {
				let y_pos = y_start + (y * tile_height);

				// crop_imm returns immutable view into the image (SubImage type)
				// .to_image fn converts the view into an ImageBuffer
				let cropped_img = image::imageops::crop_imm(&grayscale_img, 
															x_pos, 
															y_pos, 
															tile_width, 
															tile_height).to_image();

				// borrows ImageBuffer and returns a new ImageBuffer
				let rezized_img = image::imageops::resize(&cropped_img, 
														  EXPORT_TILE_WIDTH, 
														  export_tile_height, 
														  image::imageops::FilterType::Gaussian);

				let mut pixels: Vec<f32> = Vec::new();
				// rerange PIXEL_MIN – PIXEL_MAX pixel range to NEW_PIXEL_MIN – NEW_PIXEL_MAX range
				// most likely from 0.0 – 255.0 to -1.0 - 1.0 (to fit GANs latent space representation)
				for pixel in rezized_img.pixels()  { 
					let pix_val = pixel.0[0] as f32;
					pixels.push(lerp_range(&pix_val, &PIXEL_MIN, &PIXEL_MAX, &NEW_PIXEL_MIN, &NEW_PIXEL_MAX));
				}
				// shadow the pixels var
				// convert the float values to strings
			    let pixels : Vec<String> = pixels.into_iter() // array to iterator
			    						   .map(|v| v.to_string()) // convert float to string
			    						   .collect(); // iterator to collection
	    		// write the tile data to dedicated csv writer
	    		let writer_index: usize = (x * GRID_ROWS + y) as usize;
	    		writers[writer_index].write_record(&pixels).unwrap();

	    		// create dir for each tile
	    		let tile_images_folder_path: String = format!("{}{}", IMAGE_OUT_PATH_ROOT, writer_index.to_string());
	    		fs::create_dir_all(&tile_images_folder_path).unwrap();
	    		let image_path = format!("{}/{}.jpg", tile_images_folder_path, image_index.to_string());
				cropped_img.save(&image_path).unwrap();

			}
		}
	};

	for mut writer in writers {
		writer.flush().unwrap();
	}
	println!("{}", "done!");

}
