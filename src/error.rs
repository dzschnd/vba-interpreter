use core::fmt::Display;

pub fn print_err(e: impl Display) {
    println!("Billy did NOT approve this ({})\n", e)
}
