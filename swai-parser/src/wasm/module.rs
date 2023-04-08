use super::types::FunctionType;

#[derive(Debug)]
pub struct WasmModule {
    sections: WasmSections,
}

#[derive(Debug)]
pub struct WasmSections {
    custom: Vec<()>,
    types: Vec<FunctionType>,
    imports: Vec<()>,
    functions: Vec<()>,
    tables: Vec<()>,
    memory: Vec<()>,
    global: Vec<()>,
    export: Vec<()>,
    start: Option<()>,
    element: Vec<()>,
    code: Vec<()>,
    data: Vec<()>,
    data_count: Option<u32>,
}
