use std::error::Error;
use csv::Writer;

fn write_csv() -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_path("foo.csv")?;
    wtr.write_record(&[1, 2, 3])?;
    wtr.write_record(&["x", "y", "z"])?;
    wtr.flush()?;
    Ok(())
}

fn main() {
    write_csv().unwrap();
}
