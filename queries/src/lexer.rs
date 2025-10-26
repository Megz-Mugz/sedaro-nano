use crate::token_category::TokenCategory;

#[derive(Debug)]
pub struct Token {
    pub category: TokenCategory,
    pub lexeme: String,
}

pub fn tokenize_query(user_query: &str, tokens: &mut Vec<Token>) {
    let mut current_word = String::new();

    for letter in user_query.chars() {
        // Handle punctuation first
        match letter {
            '(' => {
                flush_if_identifier(&mut current_word, tokens);
                tokens.push(Token { category: TokenCategory::LParen, lexeme: "(".to_string() });
                continue;
            },
            ')' => {
                flush_if_identifier(&mut current_word, tokens);
                tokens.push(Token { category: TokenCategory::RParen, lexeme: ")".to_string() });
                continue;
            },
            ',' => {
                flush_if_identifier(&mut current_word, tokens);
                tokens.push(Token { category: TokenCategory::Comma, lexeme: ",".to_string() });
                continue;
            },
            '.' => {
                flush_if_identifier(&mut current_word, tokens);
                tokens.push(Token { category: TokenCategory::Dot, lexeme: ".".to_string() });
                continue;
            },
            ' ' | '\n' | '\t' => {
                // Treat whitespace as a boundary
                flush_if_identifier(&mut current_word, tokens);
                continue;
            },
            _ => {}
        }

        // Add character if it's not punctuation or whitespace
        current_word.push(letter);

        match current_word.as_str() {
            "prev!" => {
                tokens.push(Token { category: TokenCategory::Prev, lexeme: current_word.clone() });
                current_word.clear();
            },
            "root!" => {
                tokens.push(Token { category: TokenCategory::Root, lexeme: current_word.clone() });
                current_word.clear();
            },
            "agent!" => {
                tokens.push(Token { category: TokenCategory::Agent, lexeme: current_word.clone() });
                current_word.clear();
            },
            _ => {}
        }
    }

    // Handle any leftover identifier at end of query
    flush_if_identifier(&mut current_word, tokens);

    // End of input
    tokens.push(Token { category: TokenCategory::EOF, lexeme: "".to_string() });
}

// Flushes the current word if it's a valid identifier.
fn flush_if_identifier(current_word: &mut String, tokens: &mut Vec<Token>) {
    let trimmed = current_word.trim();
    if !trimmed.is_empty() {
        validate_identifier(trimmed);
        tokens.push(Token {
            category: TokenCategory::Identifier,
            lexeme: trimmed.to_string(),
        });
    }
    current_word.clear();
}

// Checks that an identifier follows the correct lexical rules.

fn validate_identifier(identifier: &str) {
    if identifier.is_empty() {
        panic!("Identifier cannot be empty");
    }

    for (index, letter) in identifier.chars().enumerate() {
        if index == 0 && !letter.is_alphabetic() {
            panic!("Identifier must start with a letter");
        } else if !letter.is_alphanumeric() && letter != '_' {
            panic!("Identifier can only contain alphanumeric characters and underscores");
        }
    }
}