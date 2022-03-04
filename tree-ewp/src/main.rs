
use std::path::PathBuf;
use std::fs;

use clap::Parser;
use wasmtime::{self, Engine, Module, Store, Linker};

mod interface;
mod printer;

/// Prints out the parse tree using the given EWP parser and input
#[derive(Parser)]
struct Args {
    /// The path to the EWP WASM file for the desired parser
    ewp_path: PathBuf,
    /// The input to parse with the parser
    input_path: PathBuf
}


fn main() {
    let args = Args::parse();

    // Setup WASMTIME
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    let mut store = Store::new(&engine, interface::Parser1Data {});

    // Load and initialize our EWP module
    let wat = fs::read(args.ewp_path)
        .expect("Could not read EWP WASM module file");
    let module = Module::new(&engine, wat)
        .expect("Could not initialize module");
    let (ewp, _instance) = interface::Parser1::instantiate(&mut store, &module, &mut linker, get_whole_store)
        .expect("Failed to instantiate interface");


    // Read input file and parse it
    let input = fs::read_to_string(args.input_path)
        .expect("Could not read pares input file");
    let output = ewp.parse(&mut store, &input)
        .expect("Failed to run EWP");

    printer::pretty_print(output);
}

fn get_whole_store<'a, T>(input: &'a mut T) -> &'a mut T {
    input
}
