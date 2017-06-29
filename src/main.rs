extern crate protoparse;

use std::path::Path;

use protoparse::parser::parse;

// main (for manual testing)
fn main() {
    let path = Path::new("/Users/mb/test.proto");
    let result = parse(path);

    println!("result = {:?}", result);
}
