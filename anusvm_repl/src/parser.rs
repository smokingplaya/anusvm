use anusvm_core::{ByteCode, ByteCodeInstruction, Function, Object};
use crate::lexer::Token;

#[derive(Debug, Clone)]
pub struct Parser {}

impl Parser {
  pub fn new() -> Self {
    Self {}
  }
}

impl Parser {
  pub fn parserize(&mut self, tokens: &[Token], write_buffer: &mut ByteCode, cursor: Option<usize>) -> Result<usize, ParserError> {
    let mut cursor = cursor.unwrap_or_default();
    let len = tokens.len();

    while cursor < len {
      let token = tokens.get(cursor).unwrap();

      match token {
        Token::Keyword(keyword) => {
          cursor += 1;

          match keyword.as_str() {
            "add" => self.parse_two_num(&mut cursor, tokens, write_buffer, ByteCodeInstruction::Add)?,
            "sub" => self.parse_two_num(&mut cursor, tokens, write_buffer, ByteCodeInstruction::Sub)?,
            "addf" => self.parse_two_float(&mut cursor, tokens, write_buffer, ByteCodeInstruction::AddF)?,
            "subf" => self.parse_two_float(&mut cursor, tokens, write_buffer, ByteCodeInstruction::SubF)?,
            "alloc" => self.parse_alloc(&mut cursor, tokens, write_buffer)?,
            "def" => self.parse_definefn(&mut cursor, tokens, write_buffer)?,
            "getfn" => self.parse_getfn(&mut cursor, tokens, write_buffer)?,
            "call" => write_buffer.push(ByteCodeInstruction::Call),
            _ => {}
          }
        },
        value => return Err(ParserError::InvalidSyntax(format!("Invalid token {} position", value.to_string())))
      }

      cursor += 1;
    }

    Ok(cursor)
  }
}

impl Parser {
  fn parse_two_args_internal<T: Copy + Default, F: Fn(&Token) -> Result<T, ParserError>>(&self, cursor: &mut usize, tokens: &[Token], predicate: F) -> Result<[T; 2], ParserError> {
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

  fn parse_two_num(&mut self, cursor: &mut usize, tokens: &[Token], buffer: &mut ByteCode, command: ByteCodeInstruction) -> Result<(), ParserError> {
    let arg_buffer = self.parse_two_args_internal::<isize, _>(cursor, tokens, |token| {
      match token {
        Token::Number(number) => Ok(*number),
        value => Err(ParserError::InvalidArgument(format!("expected Number, found {}", value.to_string())))
      }
    })?;

    buffer.push(ByteCodeInstruction::Push(Object::Number(arg_buffer[0])));
    buffer.push(ByteCodeInstruction::Push(Object::Number(arg_buffer[1])));

    buffer.push(command);

    Ok(())
  }

  fn parse_two_float(&mut self, cursor: &mut usize, tokens: &[Token], buffer: &mut ByteCode, command: ByteCodeInstruction) -> Result<(), ParserError> {
    let arg_buffer = self.parse_two_args_internal::<f32, _>(cursor, tokens, |token| {
      match token {
        Token::Float(float) => Ok(*float),
        value => Err(ParserError::InvalidArgument(format!("expected Float, found {}", value.to_string())))
      }
    })?;

    buffer.push(ByteCodeInstruction::Push(Object::Float(arg_buffer[0])));
    buffer.push(ByteCodeInstruction::Push(Object::Float(arg_buffer[1])));

    buffer.push(command);

    Ok(())
  }

  fn parse_alloc(&mut self, cursor: &mut usize, tokens: &[Token], buffer: &mut ByteCode) -> Result<(), ParserError> {
    let bytes = match &tokens[*cursor] {
      Token::Number(num) => Ok(num),
      value => Err(ParserError::InvalidArgument(format!("expected Number, found {}", value.to_string())))
    }?;

    if bytes <= &0 {
      return Err(ParserError::InvalidArgument("byte count should be more than 0".to_string()));
    }

    *cursor += 1;

    buffer.push(ByteCodeInstruction::Push(Object::Number(*bytes)));
    buffer.push(ByteCodeInstruction::Alloc);

    Ok(())
  }

  fn parse_definefn(&mut self, cursor: &mut usize, tokens: &[Token], buffer: &mut ByteCode) -> Result<(), ParserError> {
    let func_name = match tokens.get(*cursor) {
      Some(func_name) => match func_name {
        Token::Identifier(name) => name.to_owned(),
        value => return Err(ParserError::InvalidArgument(format!("first argument should be function name identifier but found {}", value.to_string())))
      },
      None => return Err(ParserError::InvalidArgument("first argument should be function identifier".to_string()))
    };

    *cursor += 1;

    // tokens - (Keyword(definefn) + Identifier(func_name)) that we have already take
    // more than enough
    let mut func_body = Vec::with_capacity(tokens.len() - 2);
    let new_cursor = self.parserize(tokens, &mut func_body, Some(*cursor))?;

    buffer.push(ByteCodeInstruction::DefFunc(Function::new(func_name, None, func_body)));

    *cursor = new_cursor;

    Ok(())
  }

  fn parse_getfn(&mut self, cursor: &mut usize, tokens: &[Token], buffer: &mut ByteCode) -> Result<(), ParserError> {
    let func_name = match tokens.get(*cursor) {
      Some(func_name) => match func_name {
        Token::Identifier(name) => name.to_owned(),
        value => return Err(ParserError::InvalidArgument(format!("first argument should be function name identifier but found {}", value.to_string())))
      },
      None => return Err(ParserError::InvalidArgument("first argument should be function identifier".to_string()))
    };

    *cursor += 1;

    buffer.push(ByteCodeInstruction::GetFunc(func_name));

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