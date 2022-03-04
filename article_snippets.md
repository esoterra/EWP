## Snippets
The original text for snippets in the article
Styled using the VS Code theme

```rust
parse: function(input: string) -> string
```

```rust
record token {
  label: string,
  span: span
}

record span {
  offset: u32, // The byte index where the token starts
  length: u32, // The number of bytes long the token is
}
```

```rust
record branch {
  label: string,
  // The nodes that this branch contains
  children: list<node-index>
}

variant node-index {
  token(u32),  // refers to the Nth token in output
  branch(u32), // refers to the Nth branch in output
}

record output {
  tokens: list<token>,
  tree: list<node>
}
```

```rust
parse: function(input: string) -> output
```

```rust
wit_bindgen_rust::export!("../parser1.wit");

struct Parser1 {}

impl parser1::Parser1 for Parser1 {
  fn parse(input: String) -> parser1::Output {
    let tokens = tokenize(&input);
    let tree = parse(&tokens);
    parser1::Output { tokens, tree }
  }
}

pub fn tokenize(input: &str) -> Vec<Token> {
  ...
}

pub fn parse(tokens: &[Token]) -> Vec<Branch> {
  ...
}
```

```lisp
$ cargo run -- ../ewp_json.wasm ../test.json
(Object
    (LBrace)
    (Entry
        (String)
        (Colon)
        (Number)
    )
    (RBrace)
)
```

```toml
[package]
name = "tree-ewp"
version = "0.1.0"
edition = "2021"

[dependencies]
wasmtime = "0.33.0"
wit-bindgen-wasmtime = { ... }
...
```

```rust
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
```