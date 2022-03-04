
use crate::interface;

pub fn pretty_print(output: interface::Output) {
    print_branch(&output, output.tree.len() - 1, 0);
}

fn print_branch(output: &interface::Output, index: usize, indent: u32) {
    let node = &output.tree[index];
    print_indent(indent);
    println!("({}", node.label);
    for child in node.children.iter() {
        match child {
            interface::NodeIndex::Token(t_index) =>
                print_token(output, *t_index as usize, indent + 1),
            interface::NodeIndex::Branch(b_index) =>
                print_branch(output, *b_index as usize, indent + 1),
        }
    }
    print_indent(indent);
    println!(")");
}

fn print_token(output: &interface::Output, index: usize, indent: u32) {
    let token = &output.tokens[index];
    print_indent(indent);
    println!("({})", token.label);
}

fn print_indent(indent: u32) {
    for _ in 0..indent {
        print!("  ");
    }
}