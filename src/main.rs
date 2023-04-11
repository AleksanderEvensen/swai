use std::fs::File;

use swai_parser::WasmModule;

fn main() {
    let mut add_file = File::open("./tests/add.wasm").unwrap();

    let module = WasmModule::from_file(&mut add_file).unwrap();

    println!("Module: \n{:#?}", module);
}
