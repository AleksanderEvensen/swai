use std::fs::File;
use swap;

fn main() {
    let mut add_file = File::open("./tests/add.wasm").unwrap();
    let data = swap::parse(&mut add_file).unwrap();

    println!("{:#?}", data)
}
