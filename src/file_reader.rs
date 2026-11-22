use std::io::Error;
use std::{fs::File, io::Read};

pub fn read_file(path: &str) -> Result<String, Error> {
    let mut f = File::open(path)?;
    let mut text = String::new();
    Read::read_to_string(&mut f, &mut text)?;

    Ok(text)
}
