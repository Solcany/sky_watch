use gphoto2::{Context, Result};
use std::{path, fs, thread, time};
use std::fs::create_dir_all;
use image;

const EXPORT_FOLDER : &str = "./images/";
const DELAY_SECONDS : u32 = 4;

fn main() -> Result<()> {
  // create all export dirs if they don't exist
  create_dir_all(EXPORT_FOLDER).unwrap();
  let context = Context::new()?;
  //let delay = time::Duration::from_secs(DELAY_SECONDS as u64);
  let camera = context.autodetect_camera().wait()?;

  println!({}, "capturing");

  let captured_file_path = camera
    .capture_image()
    .wait()?;

  let captured_file = camera
    .fs()
    .download(&captured_file_path.folder(), &captured_file_path.name())
    .wait()
    .unwrap();

  //let data = preview.get_data(&context).wait()?;  

  //let preview = camera.capture_preview().wait()?;
  println!({}, "getting bytes");  
  let bytes = captured_file
    .get_data(&context)
    .wait()?;  

  match image::load_from_memory_with_format(&bytes, image::ImageFormat::Jpeg) {
    Ok(img) => {
        fs::write("output.jpg", &bytes).unwrap();
    }
    Err(_) => {
        println!("input is not png");
    }
  }

  println!("Data size: {}", bytes.len());
  //let export_path : String = format!("{}/{}", EXPORT_FOLDER, &file.name().to_string());

  // let mut data = camera
  //   .fs()
  //   .download(&file.folder(), &file.name())
  //   .wait()?;

  // data.get_data(&camera);


  //println!("Downloaded image {}", &export_path);
  //count += 1;



  // loop {
  //     if count < 3 {
  //       let file = camera.capture_image().wait()?;
  //       let export_path : String = format!("{}/{}", EXPORT_FOLDER, &file.name().to_string());

  //       camera
  //         .fs()
  //         .download_to(&file.folder(), &file.name(), path::Path::new(&export_path))
  //         .wait()?;

  //       println!("Downloaded image {}", &export_path);
  //       count += 1;
  //       thread::sleep(delay);

  //     } else {
  //       println!("finished!");        
  //       break;
  //     }
  // }

  Ok(())
}
