use std::fs::File;
use std::io::{Error, Read};

fn main() {
    match read_file(String::from("test.vba")) {
        Ok(_f) => {}
        Err(e) => println!("Billy did NOT approve this ({})\n", e),
    }
}

fn read_file(path: String) -> Result<(), Error> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    Read::read_to_string(&mut f, &mut s)?;
    print!("{}", s);
    Ok(())
}
