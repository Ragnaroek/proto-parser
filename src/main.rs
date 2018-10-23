extern crate protoparser;

use std::path::Path;
use std::fs::File;
use std::io::Read;

use protoparser::parser::parse;

// main (for manual testing)
fn main() {
    let path = Path::new("/Users/mb/test.proto");
    let mut buffer = String::new();
    File::open(path).unwrap().read_to_string(&mut buffer).unwrap();
    let result = parse(&buffer);

    println!("result = {:?}", result);
}
