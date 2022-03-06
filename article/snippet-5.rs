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