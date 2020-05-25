#[macro_use]
extern crate lazy_static;

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
    use parser::parser::{parse};
    use unidiff::PatchSet;

    let diff = DiffString::from_file("./fixtures/rustball.diff")?;

    // let out = parse(diff);
    let mut patch = PatchSet::new();
    patch.parse(diff).ok().expect("Error parsing diff");

    let mut patchset_iter = patch.into_iter();
    let mut f = patchset_iter.nth(0).unwrap();
    let mut h = f.into_iter().nth(0).unwrap();
    println!("{:?}", h);
    println!("{:?}", h.into_iter().nth(0).unwrap());

    Ok(())
}
