// Setup WASMTIME
let engine = Engine::default();
let mut linker = Linker::new(&engine);
let mut store = Store::new(&engine, interface::Parser1Data {});

// Load and initialize our EWP module
let wat = fs::read(args.ewp_path)
    .expect("Could not read EWP WASM module file");
let module = Module::new(&engine, wat)
    .expect("Could not initialize module");
let result = interface::Parser1::instantiate(
  		&mut store, &module,
  		&mut linker, get_whole_store
	)
    .expect("Failed to instantiate interface");


// Read input file and parse it
let input = fs::read_to_string(args.input_path)
    .expect("Could not read pares input file");
let output = result.1.parse(&mut store, &input)
    .expect("Failed to run EWP");

printer::pretty_print(output);
