use std::env::args;
use std::fs::read_to_string;

pub fn get_content() -> std::io::Result<String> {
    let mut binder = args();
    let _ = binder.next();

    read_to_string(binder.next().unwrap())
}
