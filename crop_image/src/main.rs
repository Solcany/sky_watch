use image;

fn get_arr_size(w: &u32, h: &u32) -> usize {
	let size = w * h;
	size as usize
}

fn main() {
	const IMAGE_PATH : &str = "./images/test.jpg";
	const OUT_PATH : &str = "./images/gray.jpg";
	const CROP_X: u32 = 0;
	const CROP_Y: u32 = 0;
	const CROP_WIDTH : u32 = 10;
	const CROP_HEIGHT : u32 = 10;
	const SIZE: usize = 100;

	// load image
	let img = image::open(IMAGE_PATH).unwrap();

	// grayscale fn returns ImageBuffer
	let grayscale_img = image::imageops::colorops::grayscale(&img);

	// crop_imm returns immutable view into the image (SubImage type)
	// .to_image fn converts the view into an ImageBuffer
	let cropped_img = image::imageops::crop_imm(&grayscale_img, CROP_X, CROP_Y, CROP_WIDTH, CROP_HEIGHT).to_image();


	let mut values :[u8; SIZE] = [0; SIZE];
	for (i, pixel) in cropped_img.pixels().enumerate() {
		values[i] = pixel.0[0];
	}
	println!("{}", values[0]);

	//cropped_img.save(OUT_PATH).unwrap();
}
