use gphoto2::{Context, Result};
use std::path::Path;
use std::fs;

const EXPORT_FOLDER : &str = "./images/";

fn main() -> Result<()> {
  // create all export dirs if they don't exist
  std::fs::create_dir_all(EXPORT_FOLDER).unwrap();

  let camera = Context::new()?.autodetect_camera().wait()?;
  let file = camera.capture_image().wait()?;
  let export_path : String = format!("{}/{}", EXPORT_FOLDER, &file.name().to_string());

  camera
    .fs()
    .download_to(&file.folder(), &file.name(), Path::new(&export_path))
    .wait()?;
  println!("Downloaded image {}", file.name());

  Ok(())
}
