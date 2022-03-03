wit_bindgen_rust::export!("../interfaces/parser1.wit");

struct Parser1 {}

impl parser1::Parser1 for Parser1 {
    fn parse(input: String) -> parser1::Output {
        let tokens = tokenize(&input);
        let tree = parse(&tokens);
        parser1::Output { tokens, tree }
    }
}

fn tokenize(input: &str) -> Vec<parser1::Token> {
    let mut tokens = Vec::new();
    let mut iter = input.char_indices().peekable();
    while let Some((i, c)) = iter.next() {
        // Handle simple 1-character tokens
        let simple_token = match c {
            '\"' => Some("Quote"),
            '[' => Some("LBracket"),
            ']' => Some("RBracket"),
            '{' => Some("LBrace"),
            '}' => Some("RBrace"),
            ':' => Some("Colon"),
            ',' => Some("Comma"),
            _ => None
        };
        if let Some(token_label) = simple_token {
            tokens.push(parser1::Token {
                label: token_label.into(),
                span: parser1::Span { offset: i as u32, length: 1 }
            });
            continue;
        }
        // Handle whitespace
        if c.is_whitespace() {
            let mut len = 1;
            while let Some((j, c2)) = iter.peek() {
                if c2.is_whitespace() {
                    iter.next();
                    len += 1;
                } else {
                    break;
                }
            }
            tokens.push(parser1::Token {
                label: "Whitespace".into(),
                span: parser1::Span { offset: i as u32, length: len }
            });
            break;
        }
        // 
    }
    tokens
}

fn parse(tokens: &[parser1::Token]) -> Vec<parser1::Branch> {
    let mut parser = JSONParser::new(tokens);
    // For now we don't care if the parse errored
    let _ = parser.parse_value();
    parser.tree
}


struct JSONParser<'t> {
    tokens: &'t [parser1::Token],
    tree: Vec<parser1::Branch>,
    index: usize
}

#[derive(Clone, Copy)]
struct Checkpoint {
    index: usize,
    tree_len: usize
}

impl<'t> JSONParser<'t> {
    fn new(tokens: &'t [parser1::Token]) -> Self {
        JSONParser {
            tokens,
            tree: Vec::new(),
            index: 0
        }
    }

    fn checkpoint(&self) -> Checkpoint {
        Checkpoint {
            index: self.index,
            tree_len: self.tree.len()
        }
    }

    fn restore(&mut self, checkpoint: Checkpoint) {
        self.index = checkpoint.index;
        while self.tree.len() > checkpoint.tree_len {
            self.tree.pop();
        }
    }

    fn match_token(&mut self, label: impl Into<String>, checkpoint: Checkpoint) -> Result<parser1::NodeIndex, ()> {
        if let Some(token) = self.tokens.get(self.index) {
            if token.label == label.into() {
                let i = self.index;
                self.index += 1;
                return Ok(parser1::NodeIndex::Token(i as u32))
            }
        }
        self.restore(checkpoint);
        Err(())
    }

    fn make_branch(&mut self, label: impl Into<String>, children: Vec<parser1::NodeIndex>) -> parser1::NodeIndex {
        let node_index = parser1::NodeIndex::Branch(self.tree.len() as u32);
        self.tree.push(parser1::Branch {
            label: label.into(),
            children
        });
        node_index
    }

    fn parse_value(&mut self) -> Result<parser1::NodeIndex, ()> {
        let checkpoint = self.checkpoint();
        if let Ok(node_index) = self.match_token("String", checkpoint) {
            return Ok(node_index)
        }
        if let Ok(node_index) = self.match_token("Number", checkpoint) {
            return Ok(node_index)
        }
        if let Ok(node_index) = self.parse_object() {
            return Ok(node_index);
        }
        if let Ok(node_index) = self.parse_list() {
            return Ok(node_index);
        }
        if let Ok(node_index) = self.match_token("Null", checkpoint) {
            return Ok(node_index)
        }
        if let Ok(node_index) = self.match_token("True", checkpoint) {
            return Ok(node_index)
        }
        if let Ok(node_index) = self.match_token("False", checkpoint) {
            return Ok(node_index)
        }
        Err(())
    }

    fn parse_object(&mut self) -> Result<parser1::NodeIndex, ()> {
        let checkpoint = self.checkpoint();

        // Parse open brace
        let open_brace_i = self.match_token("LBrace", checkpoint)?;
        let mut children = vec![open_brace_i];

        loop {
            // Parse entry key string
            let key_i = self.match_token("String", checkpoint)?;
            // Parse entry colon
            let colon_i = self.match_token("Colon", checkpoint)?;

            // Parse entry value
            let value_i = match self.parse_value() {
                Ok(node_index) => node_index,
                Err(_) => {
                    self.restore(checkpoint);
                    return Err(());
                }
            };

            // Create Entry Node
            let branch_i = self.make_branch("Entry", vec![key_i, colon_i, value_i]);
            children.push(branch_i);

            // Check if there is a comma
            let comma_checkpoint = self.checkpoint();
            let comma_i = match self.match_token("Comma", comma_checkpoint) {
                Ok(index) => index,
                Err(_) => break
            };
            children.push(comma_i);
        }

        // Parse close brace
        let open_brace_i = self.match_token("RBrace", checkpoint)?;
        children.push(open_brace_i);

        // Create Object Node
        Ok(self.make_branch("Object", children))
    }

    fn parse_list(&mut self) -> Result<parser1::NodeIndex, ()> {
        let checkpoint = self.checkpoint();

        // Parse open brace
        let open_brace_i = self.match_token("LBracket", checkpoint)?;
        let mut children = vec![open_brace_i];

        loop {
            // Parse value
            let value_i = match self.parse_value() {
                Ok(node_index) => node_index,
                Err(_) => {
                    self.restore(checkpoint);
                    return Err(());
                }
            };
            children.push(value_i);

            // Check if there is a comma
            let comma_checkpoint = self.checkpoint();
            let comma_i = match self.match_token("Comma", comma_checkpoint) {
                Ok(index) => index,
                Err(_) => break
            };
            children.push(comma_i);
        }

        // Parse close brace
        let open_brace_i = self.match_token("RBracket", checkpoint)?;
        children.push(open_brace_i);

        // Create Object Node
        Ok(self.make_branch("List", children))
    }
}
