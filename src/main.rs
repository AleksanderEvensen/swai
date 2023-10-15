use std::fs::File;
use swai_parser::WasmModule;

use crate::interpreter::WasmEnvironment;

mod error;
mod interpreter;

fn main() {
    let mut add_file = File::open("./tests/helloworld.wasm").unwrap();

    let module = WasmModule::from_file(&mut add_file).unwrap();
    println!("Module: \n{:#?}", module);

    let mut memory: [u8; 32] = [0; 32];

    let mut env = WasmEnvironment::new(module, &mut memory);

    env.start();
}
