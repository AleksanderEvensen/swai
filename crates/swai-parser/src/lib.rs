pub mod error;
pub mod instructions;
mod leb128;
pub mod sections;
pub mod types;
pub mod wasm;

pub use wasm::WasmModule;
