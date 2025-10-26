#![allow(warnings)]

use std::io;
mod lexer;
mod token_category;
mod parser;

use lexer::{tokenize_query, Token};

use crate::parser::Parser;

fn main() {
    // production rules:
    // 1. Query → "prev!" "(" Query ")"
    // 2. Query → "root!"
    // 3. Query → "agent!" "(" ID ")"
    // 4. Query → ID
    // 5. Query → "(" QueryList ")"
    // 6. Query → Query "." ID
    // 7. QueryList → Query
    // 8. QueryList → Query "," QueryList

    // Steps to make the language compiled:
    //
    // 1. Lexical analysis (tokenization):
    //    - Break the input into tokens like <category, lexeme> pairs.
    //    - Validate that each token matches valid lexical patterns.

    println!("Please enter your query or queries of choice: ");

    let mut user_query: String = String::new();
    let mut tokens: Vec<Token> = Vec::new();

    io::stdin().read_line(&mut user_query).expect("Failed to read line");

    tokenize_query(&user_query, &mut tokens);

    
    // for token in &tokens {
    //     print!("{:?}\n", token);
    // }
    
    // 2. Parsing (syntactic analysis):
    //    - Validate that the sequence of tokens conforms to the grammar (syntactic correctness).
    //    - Only if the syntax is valid, construct the parse tree (AST structure begins here).
    let mut parser= Parser::new(tokens);
    parser.parse_query();

    // going to skip semantic analysis for now
    
    
    // 4. Serialization:
    //    - Convert the final AST into JSON or another intermediate representation.
    //
    // 5. Output:
    //    - Save the serialized AST as `output.ast` (or pass it to a backend for code generation).


}
