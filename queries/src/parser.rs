use crate::lexer::Token;

use crate::token_category::TokenCategory;




pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, position: 0 }
    }
    
    fn look_at_current_token(&self) -> &TokenCategory {
        &self.tokens[self.position].category
    }

    fn advance_pointer(&mut self) {
        if self.position + 1 < self.tokens.len() {
            self.position += 1;
        }
    }

    fn validate_current_and_move_to_next_token(&mut self, expected_token: TokenCategory) {
        if *self.look_at_current_token() != expected_token {
            panic!(
                "Syntax Error: Expected {:?}, found {:?}",
                expected_token,
                self.look_at_current_token()
            );
        }
        
        self.advance_pointer();
    }

    pub fn parse_query(&mut self) {
        // Placeholder function for parsing logic
        // This function would implement the parsing rules defined in main.rs
        println!("Parsing tokens...");

        // production rules:
        // 1. Query → "prev!" "(" Query ")"
        // 2. Query → "root!"
        // 3. Query → "agent!" "(" ID ")"
        // 4. Query → ID
        // 5. Query → "(" QueryList ")"
        // 6. Query → Query "." ID
        // 7. QueryList → Query
        // 8. QueryList → Query "," QueryList

        match self.look_at_current_token() {
            TokenCategory::Prev => {
                println!("Matched 'prev!'");
                // expect prev on current & move to next
                self.validate_current_and_move_to_next_token(TokenCategory::Prev);
                // expect a LPAREN and move to next
                self.validate_current_and_move_to_next_token(TokenCategory::LParen);
                // expect a query of some kind, so we can recursively call parse_query
                let query = self.parse_query();
                // after all the recursion, we should expect a RPAREN to close the prev query
                self.validate_current_and_move_to_next_token(TokenCategory::RParen);
                
            }
            TokenCategory::Root => {
                println!("Matched 'root!'");
                // Implement parsing logic for "root!"
            }
            TokenCategory::Agent => {
                println!("Matched 'agent!'");
                // Implement parsing logic for "agent!" "(" ID ")"
            }
            TokenCategory::Identifier => {
                println!("Matched ID");
                // Implement parsing logic for ID
            }
            TokenCategory::LParen => {
                println!("Matched '('");
                // Implement parsing logic for "(" QueryList ")"
            }
            _ => {println!("Unexpected token: {:?}", self.look_at_current_token());}
        }
    }

    
}