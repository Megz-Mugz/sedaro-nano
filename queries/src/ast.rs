use serde::{Serialize, Deserialize};

// Rust allows me to print and serialize the ASTNode enum for easy debugging and JSON serialization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ASTNode {
    Prev(Box<ASTNode>), // Ran into issue with recursive types, so had to box to give it a fixed size. 
    Root,                           
    Agent(String),                  
    Identifier(String),              
    Tuple(Vec<ASTNode>),             
    Access(Box<ASTNode>, String)
}