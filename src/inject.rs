use wasm_inject_data::wasm_transform::{DataSegment, DataSegmentKind, Module};
use wasmparser::Operator;

/// Injects a passive data segment into the given wasm module so that it can be
/// read at runtime.
fn main() {
    // load the Wasm module
    let args = std::env::args().collect::<Vec<_>>();
    let input_file = &args[1];
    let output_file = &args[2];
    let contents = std::fs::read(input_file).unwrap();
    let mut module = Module::parse(&contents, true).unwrap();

    // Add a new data segment with the provided data.
    let data = vec![1, 2, 3, 4, 5];
    let data_seg = DataSegment {
        kind: DataSegmentKind::Passive,
        data: &data,
    };
    module.data.push(data_seg);
    // Add a data count section (this seems to be required).
    module.data_count_section_exists = true;

    // Modifiy the "passive_data_size" function to return the correct size of
    // the data we injected.
    let data_size_fn_index = module
        .exports
        .iter()
        .find(|ex| ex.name == "passive_data_size")
        .unwrap()
        .index as usize;
    module.code_sections[data_size_fn_index].locals = vec![];
    module.code_sections[data_size_fn_index].instructions = vec![
        Operator::I32Const {
            value: data.len() as i32,
        },
        Operator::End,
    ];

    // Write the modified wasm to the output file.
    let result = module.encode().unwrap();
    std::fs::write(output_file, result).unwrap();
}
