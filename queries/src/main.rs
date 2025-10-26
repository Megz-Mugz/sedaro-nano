use std::io;

fn main() {
    // let input =
    //     read_to_string(stdin()).unwrap_or_else(|err| panic!("Could not read input stream! {err}"));
    // let parser = grammar::QueryParser::new();
    // let query = parser
    //     .parse(&input)
    //     .unwrap_or_else(|err| panic!("Could not parse input! {err}"));
    // let output = serde_json::to_string(&query).unwrap();


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
    //
    // 2. Parsing (syntactic analysis):
    //    - Validate that the sequence of tokens conforms to the grammar (syntactic correctness).
    //    - Only if the syntax is valid, construct the parse tree (AST structure begins here).
    //
    // 3. Semantic analysis:
    //    - Traverse the AST to ensure it makes logical sense (semantic correctness).
    //    - Example: check that identifiers exist, types match, and operator usage is valid.
    //
    // 4. Serialization:
    //    - Convert the final AST into JSON or another intermediate representation.
    //
    // 5. Output:
    //    - Save the serialized AST as `output.ast` (or pass it to a backend for code generation).

    println!("Please enter your query or queries of choice:\n");

    let mut user_query = String::new();


    io::stdin()
    .read_line(&mut user_query)
    .expect("Failed to read line");

    

    print!("Your query was: {}", user_query);

}
