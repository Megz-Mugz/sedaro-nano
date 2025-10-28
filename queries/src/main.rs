use std::io;
use std::fs;

mod lexer;
mod parser;
mod ast;
mod token_category;

use crate::parser::Parser;
use crate::lexer::Token;

fn main() {

    let output_file_name = "executable_query.json";

    if fs::metadata(output_file_name).is_ok() {
        fs::remove_file(output_file_name).expect("Failed to remove existing executable_query.json");
        println!("Removed existing executable_query.json");
    }

    println!("Please enter your query or queries of choice: ");

    let mut user_query = String::new();
    
    io::stdin()
        .read_line(&mut user_query)
        .expect("Failed to read line");

    let tokens = Token::tokenize_query(&user_query);
    
    println!("\nGenerated Tokens:");
    Token::print_tokens(&tokens);
    
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    println!("\nGenerated AST:\n{:#?}", ast);

    // Serialize to JSON
    let serialized_ast = serde_json::to_string_pretty(&ast).unwrap();

    // Write to a file
    std::fs::write("executable_query.json", &serialized_ast)
        .expect("Failed to write AST file");

    println!("\nSerialized AST - written to executable_query.json");


}