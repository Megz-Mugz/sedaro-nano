use crate::token_category::TokenCategory;

#[derive(Debug)]
pub struct Token {
    pub category: TokenCategory,
    pub lexeme: String,
}

impl Token {
    /// Tokenize a query string into a vector of Tokens
    pub fn tokenize_query(user_query: &str) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut current_word = String::new();

        for letter in user_query.chars() {
            // if a special letter or whitespace is found, flush the current word as an identifier (if valid)
            // continued after every found special letter to avoid pushing letter to the current_word
            match letter {
                '(' => {
                    Token::flush_if_identifier(&mut current_word, &mut tokens);
                    tokens.push(Token::new(TokenCategory::LParen, "("));
                    continue;
                }
                ')' => {
                    Token::flush_if_identifier(&mut current_word, &mut tokens);
                    tokens.push(Token::new(TokenCategory::RParen, ")"));
                    continue;
                }
                ',' => {
                    Token::flush_if_identifier(&mut current_word, &mut tokens);
                    tokens.push(Token::new(TokenCategory::Comma, ","));
                    continue;
                }
                '.' => {
                    Token::flush_if_identifier(&mut current_word, &mut tokens);
                    tokens.push(Token::new(TokenCategory::Dot, "."));
                    continue;
                }
                ' ' => {
                    Token::flush_if_identifier(&mut current_word, &mut tokens);
                    continue;
                }
                _ => {}
            }

            // If a keyword is found, push the corresponding token and clear current_word
            current_word.push(letter);

            match current_word.as_str() {
                "prev!" => {
                    tokens.push(Token::new(TokenCategory::Prev, &current_word));
                    current_word.clear();
                }
                "root!" => {
                    tokens.push(Token::new(TokenCategory::Root, &current_word));
                    current_word.clear();
                }
                "agent!" => {
                    tokens.push(Token::new(TokenCategory::Agent, &current_word));
                    current_word.clear();
                }
                _ => {}
            }
        }

        // Handle leftover identifier
        Token::flush_if_identifier(&mut current_word, &mut tokens);

        // End of input
        tokens.push(Token::new(TokenCategory::EOF, ""));
        tokens
    }

    // when constructing a new token
    fn new(category: TokenCategory, lexeme: &str) -> Token {
        Token {
            category,
            lexeme: lexeme.to_string(),
        }
    }

    // Flush current word if it's a valid identifier
    fn flush_if_identifier(current_word: &mut String, tokens: &mut Vec<Token>) {
        let trimmed = current_word.trim();
        if !trimmed.is_empty() {
            Token::validate_identifier(trimmed);
            tokens.push(Token::new(TokenCategory::Identifier, trimmed));
        }
        current_word.clear();
    }

    // Validate identifier syntax
    fn validate_identifier(identifier: &str) {
        if identifier.is_empty() {
            panic!("Identifier cannot be empty");
        }

        for (index, letter) in identifier.chars().enumerate() {
            // Identifier must start with a letter & after the first character, 
            // it can only contain alphanumeric characters and underscores
            if index == 0 && !letter.is_alphabetic() {
                panic!("Identifier must start with a letter: {}", identifier);
            } else if !letter.is_alphanumeric() && letter != '_' {
                panic!(
                    "Identifier can only contain alphanumeric characters and underscores: {}",
                    identifier
                );
            }
        }
    }

    // Print tokens for debugging purposes
    pub fn print_tokens(tokens: &Vec<Token>) {
        for token in tokens {
            println!("{:?}", token);
        }
    }

}