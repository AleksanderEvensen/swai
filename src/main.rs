use std::{error::Error, fs::File};
use swai_parser::WasmModule;

use crate::interpreter::WasmEnvironment;

mod error;
mod interpreter;
fn main() -> Result<(), Box<dyn Error>> {
    let mut add_file = File::open("./tests/asc_test.wasm")?;

    let module = WasmModule::from_file(&mut add_file)?;
    println!("Module: \n{:#?}", module);

    let mut memory: [u8; 2048] = [0; 2048];
    let mut env = WasmEnvironment::new(module, &mut memory);

    env.start()?;

    Ok(())
}
