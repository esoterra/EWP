
mod json_tokenize;
mod json_parse;

wit_bindgen_rust::export!("../interfaces/parser1.wit");

struct Parser1 {}

impl parser1::Parser1 for Parser1 {
    fn parse(input: String) -> parser1::Output {
        let tokens = json_tokenize::tokenize(&input);
        let tree = json_parse::parse(&tokens);
        parser1::Output { tokens, tree }
    }
}
