// NOTE: This tokenizer contains known deviations from JSON standard.
// These exist so that trees can be generated for nearly valid input
// as well as to simplify the logic for parsing.
// As such it is intended to accept all valid inputs, but not reject all invalid ones.
use super::parser1;

use std::str::CharIndices;

type PeekableChars<'i> = std::iter::Peekable<CharIndices<'i>>;

pub fn tokenize(input: &str) -> Vec<parser1::Token> {
    let mut tokens = Vec::new();
    let mut iter = input.char_indices().peekable();

    while let Some((i, c)) = iter.next() {
        // Handle simple 1-character tokens
        let simple_token = match c {
            '[' => Some("LBracket"),
            ']' => Some("RBracket"),
            '{' => Some("LBrace"),
            '}' => Some("RBrace"),
            ':' => Some("Colon"),
            ',' => Some("Comma"),
            _ => None
        };
        if let Some(token_label) = simple_token {
            println!("Handled simple at {}", i);
            tokens.push(parser1::Token {
                label: token_label.into(),
                span: parser1::Span { offset: i as u32, length: 1 }
            });
            continue;
        }
        // Handle whitespace
        if c.is_whitespace() {
            println!("Handled whitespace at {}", i);
            let mut len = 1;
            while let Some((_j, c2)) = iter.peek() {
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
            continue;
        }
        // Handle Strings
        if c == '"' {
            println!("Handled string at {}", i);
            let mut len = 1;
            loop {
                if let Some((_j, c2)) = iter.next() {
                    len += c2.len_utf8();
                    if c2 == '\\' {
                        // Skip a character whenever we see an escape
                        if let Some((_k, c3)) = iter.peek() {
                            len += c3.len_utf8();
                            iter.next();
                        }
                    }
                    if c2 == '"' {
                        break
                    }
                }
            }
            tokens.push(parser1::Token {
                label: "String".into(),
                span: parser1::Span { offset: i as u32, length: len as u32 }
            });
            continue;
        }
        // Handle numbers
        if c == '-' || c.is_numeric() {
            println!("Handled number at {}", i);
            let mut len = 1;
            // Consume leading digits
            len += consume_digits(&mut iter);
            // Consume fractional part
            if let Some((_j, c2)) = iter.peek() {
                if *c2 == '.' {
                    // skip and count the period
                    let _ = iter.next(); 
                    len += 1;
                    // skip and count the digits
                    len += consume_digits(&mut iter);
                }
            }
            // Consume the E-notation label
            match iter.peek().map(|(_k, c)| *c) {
                Some('e' | 'E') => {
                    let _ = iter.next(); 
                    len += 1;

                    // Consume the E-notation sign
                    match iter.peek().map(|(_k, c3)| *c3) {
                        Some('-' | '+') => {
                            let _ = iter.next(); 
                            len += 1;
                        },
                        _ => {}
                    }

                    len += consume_digits(&mut iter);
                },
                _ => {}
            }

            tokens.push(parser1::Token {
                label: "Number".into(),
                span: parser1::Span { offset: i as u32, length: len as u32 }
            });
            continue;
        }
        // Handle true
        if c == 't' && matches_value(&mut iter, "rue") {
            println!("Handled true at {}", i);
            tokens.push(parser1::Token {
                label: "True".into(),
                span: parser1::Span { offset: i as u32, length: 4 }
            });
            continue;
        }
        // Handle false
        if c == 'f' && matches_value(&mut iter, "alse") {
            println!("Handled false at {}", i);
            tokens.push(parser1::Token {
                label: "False".into(),
                span: parser1::Span { offset: i as u32, length: 5 }
            });
            continue;
        }
        // Handle null
        if c == 'n' && matches_value(&mut iter, "ull") {
            println!("Handled null at {}", i);
            tokens.push(parser1::Token {
                label: "Null".into(),
                span: parser1::Span { offset: i as u32, length: 4 }
            });
            continue;
        }
        println!("Made it to end")
    }
    tokens
}

fn consume_digits(iter: &mut PeekableChars) -> u32 {
    let mut consumed = 0;
    while let Some((_i, c)) = iter.peek() {
        if !c.is_numeric() {
            break;
        }
        consumed += c.len_utf8();
        iter.next();
    }
    consumed as u32
}

fn matches_value(iter: &mut PeekableChars, value: &str) -> bool {
    for c in value.chars() {
        if Some(c) != iter.next().map(|c| c.1) {
            return false;
        }
    }
    true
}

impl PartialEq for parser1::Token {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
        && self.span.offset == other.span.offset
        && self.span.length == other.span.length
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

    #[test]
    fn test_example() {
        let input = r#"[{"a":1}, true,  null,false]"#;
        let output = vec![
            token(0,  1, "LBracket"),
            token(1,  1, "LBrace"),
            token(2,  3, "String"),
            token(5,  1, "Colon"),
            token(6,  1, "Number"),
            token(7,  1, "RBrace"),
            token(8,  1, "Comma"),
            token(9,  1, "Whitespace"),
            token(10, 4, "True"),
            token(14,  1, "Comma"),
            token(15,  2, "Whitespace"),
            token(17,  4, "Null"),
            token(21,  1, "Comma"),
            token(22,  5, "False"),
            token(27,  1, "RBracket"),
        ];
        assert_eq!(tokenize(&input), output);
    }
}