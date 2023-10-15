use swai_parser::{types::SegmentMode, WasmModule};

use crate::error::WasmInterpreterError;

#[derive(Debug)]
pub struct WasmEnvironment<'a> {
    module: WasmModule,
    memory: &'a mut [u8],
}

impl WasmEnvironment<'_> {
    pub fn new<'a>(module: WasmModule, memory: &'a mut [u8]) -> WasmEnvironment<'a> {
        WasmEnvironment { module, memory }
    }
}

impl WasmEnvironment<'_> {
    /// Executes the entry point 'start' method
    pub fn start(&mut self) -> Result<(), WasmInterpreterError> {
        // Initialize the memory
        println!("[DBG] Initialize Memory");
        for segment in self.module.sections.data.iter() {
            let (_memory_index, offset) = match &segment.mode {
                SegmentMode::Passive => (0, 0),
                SegmentMode::Active {
                    memory_index,
                    offset,
                } => {
                    let offset: usize = match offset.get(0) {
                        Some(expr) => match expr {
                            swai_parser::instructions::Instructions::i32_const(num) => {
                                *num as usize
                            }
                            swai_parser::instructions::Instructions::i64_const(num) => {
                                *num as usize
                            }

                            instr => {
                                return Err(WasmInterpreterError::InvalidMemorySegmentOffset(
                                    instr.clone(),
                                ));
                            }
                        },
                        None => 0,
                    };

                    (*memory_index, offset)
                }
            };

            let bytes = segment.bytes.clone();

            for (i, byte) in bytes.iter().enumerate() {
                let memory_len = self.memory.len();
                *self.memory.get_mut(offset + i).ok_or(
                    WasmInterpreterError::ModifyMemoryOutOfBounds {
                        offset,
                        data: bytes.clone(),
                        failed_pos: offset + i,
                        memory_len,
                    },
                )? = *byte;
            }
        }

        let Some(start_fn_index) = &self.module.sections.start else {
        	return Err(WasmInterpreterError::NoEntryPoint);
        };

        let Some(start_fn) = start_fn_index.get_function(&self.module.sections) else {
			return Err(WasmInterpreterError::NoEntryPoint);
		};

        Ok(())
    }
}
