use crate::parser1::NodeIndex;

use super::parser1;

pub fn parse(tokens: &[parser1::Token]) -> Vec<parser1::Branch> {
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

impl PartialEq for parser1::Branch {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
        && self.children == other.children
    }
}

impl PartialEq for parser1::NodeIndex {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (NodeIndex::Token(t1), &NodeIndex::Token(t2)) => *t1 == t2,
            (NodeIndex::Branch(b1), &NodeIndex::Branch(b2)) => *b1 == b2,
            _ => false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn token(offset: u32, length: u32, label: impl Into<String>) -> parser1::Token {
        parser1::Token {
            label: label.into(),
            span: parser1::Span { offset, length }
        }
    }

    fn branch(label: impl Into<String>, children: Vec<parser1::NodeIndex>) -> parser1::Branch {
        parser1::Branch {
            label: label.into(),
            children
        }
    }

    #[test]
    fn test_parse_list() {
        let input = vec![
            token(0, 1, "LBracket"),
            token(1, 1, "Number"),
            token(2, 1, "RBracket"),
        ];
        let output = vec![
            branch("List", vec![
                parser1::NodeIndex::Token(0),
                parser1::NodeIndex::Token(1),
                parser1::NodeIndex::Token(2),
            ])
        ];
        assert_eq!(parse(&input), output);
    }

    #[test]
    fn test_parse_object() {
        let input = vec![
            token(0, 1, "LBrace"),
            token(1, 3, "String"),
            token(4, 1, "Colon"),
            token(5, 1, "Number"),
            token(6, 1, "RBrace"),
        ];
        let output = vec![
            branch("Entry", vec![
                parser1::NodeIndex::Token(1),
                parser1::NodeIndex::Token(2),
                parser1::NodeIndex::Token(3),
            ]),
            branch("Object", vec![
                parser1::NodeIndex::Token(0),
                parser1::NodeIndex::Branch(0),
                parser1::NodeIndex::Token(4),
            ])
        ];
        assert_eq!(parse(&input), output);
    }

    #[test]
    fn test_parse_object_multiple() {
        let input = vec![
            token(0,  1, "LBrace"),
            token(1,  3, "String"),
            token(4,  1, "Colon"),
            token(5,  1, "Number"),
            token(6,  1, "Comma"),
            token(7,  3, "String"),
            token(10, 1, "Colon"),
            token(11, 1, "Number"),
            token(12, 1, "RBrace"),
        ];
        let output = vec![
            branch("Entry", vec![
                parser1::NodeIndex::Token(1),
                parser1::NodeIndex::Token(2),
                parser1::NodeIndex::Token(3),
            ]),
            branch("Entry", vec![
                parser1::NodeIndex::Token(5),
                parser1::NodeIndex::Token(6),
                parser1::NodeIndex::Token(7),
            ]),
            branch("Object", vec![
                parser1::NodeIndex::Token(0),
                parser1::NodeIndex::Branch(0),
                parser1::NodeIndex::Token(4),
                parser1::NodeIndex::Branch(1),
                parser1::NodeIndex::Token(8),
            ])
        ];
        assert_eq!(parse(&input), output);
    }
}
