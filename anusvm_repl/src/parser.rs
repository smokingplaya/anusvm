use anusvm_core::{ByteCode, ByteCodeInstruction, Object};
use crate::lexer::Token;

#[derive(Debug, Clone)]
pub struct Parser {
  buffer: ByteCode
}

impl Parser {
  pub fn new() -> Self {
    Self {
      buffer: Vec::with_capacity(32)
    }
  }
}

impl Parser {
  #[inline]
  pub fn buffer(&self) -> &ByteCode {
    &self.buffer
  }
}


impl Parser {
  pub fn parserize(&mut self, tokens: &Vec<Token>) -> Result<(), ParserError> {
    let mut cursor = 0;
    let len = tokens.len();

    while cursor < len {
      let token = tokens.get(cursor).unwrap();

      match token {
        Token::Keyword(keyword) => {
          cursor += 1;

          match keyword.as_str() {
            "add" => self.parse_add(&mut cursor, tokens)?,
            "alloc" => self.parse_alloc(&mut cursor, tokens)?,
            "definefn" => self.parse_definefn(&mut cursor, tokens)?,
            _ => {}
          }
        },
        Token::Identifier(_) => {},
        Token::Number(_n) => {},
        Token::Float(_) => {},
        Token::String(_) => {}
      }

      cursor += 1;
    }

    Ok(())
  }
}

impl Parser {
  fn parse_two_args<T: Copy + Default, F: Fn(&Token) -> Result<T, ParserError>>(&self, cursor: &mut usize, tokens: &[Token], predicate: F) -> Result<[T; 2], ParserError> {
    let len = tokens.len();

    let mut index = 0;
    let mut buffer: [T; 2] = [T::default(); 2];

    while *cursor < len {
      let token = tokens.get(*cursor)
        .unwrap();

      buffer[index] = predicate(token)?;

      index += 1;
      *cursor += 1;

      if index == 2 {
        break
      }
    }

    if index != 2 {
      return Err(ParserError::InvalidSyntax("command excepts two Number arguments".to_string()))
    }

    Ok(buffer)
  }

  fn parse_add(&mut self, cursor: &mut usize, tokens: &[Token]) -> Result<(), ParserError> {
    let buffer = self.parse_two_args::<isize, _>(cursor, tokens, |token| {
      match token {
        Token::Number(number) => Ok(*number),
        _ => Err(ParserError::InvalidArgument("expected Number, found unknown".to_string()))
      }
    })?;

    self.buffer.push(ByteCodeInstruction::Push(Object::Number(buffer[0])));
    self.buffer.push(ByteCodeInstruction::Push(Object::Number(buffer[1])));

    self.buffer.push(ByteCodeInstruction::Add);

    Ok(())
  }

  fn parse_alloc(&mut self, cursor: &mut usize, tokens: &[Token]) -> Result<(), ParserError> {
    let bytes = match &tokens[*cursor] {
      Token::Number(num) => Ok(num),
      value => Err(ParserError::InvalidArgument(format!("expected Number, found {}", value.to_string())))
    }?;

    if bytes.is_negative() {
      return Err(ParserError::InvalidArgument("byte count should be more than 0".to_string()));
    }

    *cursor += 1;

    self.buffer.push(ByteCodeInstruction::Alloc(*bytes as usize));

    Ok(())
  }

  fn parse_definefn(&self, cursor: &mut usize, tokens: &[Token]) -> Result<(), ParserError> {
    Ok(())
  }
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum ParserError {
  #[error("InvalidSyntax: {0}")]
  InvalidSyntax(String),
  #[error("InvalidArgument: {0}")]
  InvalidArgument(String)
}