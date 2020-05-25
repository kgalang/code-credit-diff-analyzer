use std::fs::File;
use std::io::prelude::*;

mod parser;

struct DiffString;

impl DiffString {
    fn from_file(path: &str) -> Result<String, std::io::Error> {
        let mut input = File::open(path)?;
        let mut input_buffer = String::new();
        input.read_to_string(&mut input_buffer)?;
        Ok(input_buffer)
    }
    // fn from_github
}

fn main() -> std::io::Result<()> {
    let diff = DiffString::from_file("./fixtures/large.diff")?;

    let mut lines = diff.lines();

    println!("Lines {:?}", lines.next());
    Ok(())
}
