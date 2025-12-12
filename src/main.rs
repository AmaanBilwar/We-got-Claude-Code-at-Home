use tree_sitter::Parser;

fn main() {
    let mut parser = Parser::new();
    let source_code = "fn test(a: u32) {}";

    parser
        .set_language(&tree_sitter_rust::LANGUAGE.into())
        .expect("Failed to parse language");
    let tree = parser.parse(source_code, None).unwrap();
    let root_node = tree.root_node();
    println!("root node: {}", root_node);
}
