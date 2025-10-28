use crate::lexer::Token;
use crate::token_category::TokenCategory;
use crate::ast::ASTNode;

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

    fn validate_current_and_move_to_next_token(&mut self, expected: TokenCategory) {
        if *self.look_at_current_token() != expected {
            panic!(
                "Syntax Error: Expected {:?}, found {:?}",
                expected,
                self.look_at_current_token()
            );
        }
        self.advance_pointer();
    }

    // function called in main to start parsing
    pub fn parse(&mut self) -> ASTNode {
        let queries = self.parse_query_list();

        if *self.look_at_current_token() != TokenCategory::EOF {
            panic!("Syntax Error: extra tokens found at end of input");
        }
        
        // If there's only one query in the list of queries, return it directly, 
        // otherwise return a Tuple node containing all queries.
        if queries.len() == 1 {
            queries.into_iter().next().unwrap()
        } else {
            ASTNode::Tuple(queries)
        }
    }

    // recursive descent parsing functions
    // leveraged pattern matching to make the code more readable
    fn parse_query(&mut self) -> ASTNode {

        match self.look_at_current_token() {

            TokenCategory::Prev => {
                println!("Matched 'prev!'");
                self.validate_current_and_move_to_next_token(TokenCategory::Prev);
                self.validate_current_and_move_to_next_token(TokenCategory::LParen);

                let mut node = ASTNode::Prev(Box::new(self.parse_query()));
                
                // handle the case where Query → Query . ID
                if *self.look_at_current_token() == TokenCategory::Dot {
                    self.validate_current_and_move_to_next_token(TokenCategory::Dot);
                    let field = self.tokens[self.position].lexeme.clone();
                    self.validate_current_and_move_to_next_token(TokenCategory::Identifier);
                    node = ASTNode::Access(Box::new(node), field);
                }

                self.validate_current_and_move_to_next_token(TokenCategory::RParen);
                node
            }

            TokenCategory::Root => {
                println!("Matched 'root!'");
                self.validate_current_and_move_to_next_token(TokenCategory::Root);
                ASTNode::Root
            }

            TokenCategory::Agent => {
                println!("Matched 'agent!'");
                self.validate_current_and_move_to_next_token(TokenCategory::Agent);
                self.validate_current_and_move_to_next_token(TokenCategory::LParen);

                let id_token = self.tokens[self.position].lexeme.clone();
                self.validate_current_and_move_to_next_token(TokenCategory::Identifier);
                self.validate_current_and_move_to_next_token(TokenCategory::RParen);

                let mut node = ASTNode::Agent(id_token);
                
                // you have a dot after the agent, so handle Query → Query . ID
                if *self.look_at_current_token() == TokenCategory::Dot {
                    self.validate_current_and_move_to_next_token(TokenCategory::Dot);
                    let field = self.tokens[self.position].lexeme.clone();
                    self.validate_current_and_move_to_next_token(TokenCategory::Identifier);
                    node = ASTNode::Access(Box::new(node), field);
                }

                node
            }
            TokenCategory::Identifier => {
                println!("Matched ID");
                let id = self.tokens[self.position].lexeme.clone();
                self.validate_current_and_move_to_next_token(TokenCategory::Identifier);

                // handle Query → Query . ID
                if *self.look_at_current_token() == TokenCategory::Dot {
                    self.validate_current_and_move_to_next_token(TokenCategory::Dot);
                    let field = self.tokens[self.position].lexeme.clone();
                    self.validate_current_and_move_to_next_token(TokenCategory::Identifier);
                    ASTNode::Access(Box::new(ASTNode::Identifier(id)), field)
                } else {
                    ASTNode::Identifier(id)
                }
            }
            TokenCategory::LParen => {
                println!("Matched '('");
                self.validate_current_and_move_to_next_token(TokenCategory::LParen);
                let queries = self.parse_query_list();
                self.validate_current_and_move_to_next_token(TokenCategory::RParen);
                ASTNode::Tuple(queries)
            }
            _ => {
                panic!(
                    "Unexpected token at {:?}",
                    self.look_at_current_token()
                )
            }
        }
    }

    // Assume that the user will given mutlple queries separated by commas, 
    // in the event they don't, the function will still work as intended since it won't
    // enter the while loop.
    fn parse_query_list(&mut self) -> Vec<ASTNode> {
        let mut queries = Vec::new();

        let first = self.parse_query();
        queries.push(first);

        while *self.look_at_current_token() == TokenCategory::Comma {
            self.validate_current_and_move_to_next_token(TokenCategory::Comma);
            let next_query = self.parse_query();
            queries.push(next_query);
        }

        queries
    }
}