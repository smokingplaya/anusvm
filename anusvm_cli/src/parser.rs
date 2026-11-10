use crate::lexer::Token;

pub struct Parser;

impl Parser {
  pub fn new() -> Self {
    Self {}
  }
}

impl Parser {
  pub fn parserize(&self, tokens: &Vec<Token>) -> Result<(), ParserError> {
    Ok(())
  }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ParserError {}