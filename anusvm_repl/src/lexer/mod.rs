use std::collections::HashSet;
use anyhow::anyhow;
use crate::lexer::convertable_buffer::{ConvertableBuffer, ConvertableBufferError};

mod convertable_buffer;

/// Lexem implementation
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
  Keyword(String),
  Identifier(String),
  Number(isize)
}

// allocation hater
#[derive(Debug, Clone)]
pub struct Lexer<'a> {
  keywords: HashSet<&'a str>,
  buffer: Vec<Token>,
  collectable_buffer: ConvertableBuffer,
}

/// Constructor
impl<'a> Lexer<'a> {
  #[inline(always)]
  pub fn new() -> Self {
    Self {
      keywords: HashSet::from(["add", "addf", "sub", "subf", "pushnil", "pushbool", "pushnum", "pushft", "pushstr", "pushfn", "pop", "definefn"]),
      buffer: Vec::with_capacity(16),
      collectable_buffer: ConvertableBuffer::new()
    }
  }
}

impl<'a> Lexer<'a> {
  pub fn tokenize(&mut self, input: String) -> anyhow::Result<()> {
    let mut cursor = 0;
    let len = input.len();

    self.buffer.clear();

    if len == 0 {
      return Ok(())
    }

    let mut chars = input.chars().collect::<Vec<char>>();
    while cursor < len {
      let char = chars.get(cursor).unwrap()
        .to_owned();

      if char.is_whitespace() {
        cursor += 1;
        continue;
      }

      if char.is_digit(10) {
        self.collect_number(&mut cursor, &mut chars)?;
        // safety: right above
        self.buffer.push(Token::Number(unsafe { self.collectable_buffer.convert() }));
      } else if char.is_alphabetic() {
        self.collect_ident(&mut cursor, &mut chars)?;

        let identifier = unsafe { self.collectable_buffer.convert::<String>() };

        match self.keywords.contains(identifier.as_str()) {
          true => self.buffer.push(Token::Keyword(identifier)),
          false => self.buffer.push(Token::Identifier(identifier)),
        };
      } else {
        return Err(anyhow!(LexingError::UnknownToken(char.to_owned(), cursor + 1)))
      }

      cursor += 1;
    }

    Ok(())
  }

  #[inline]
  fn collect_symbols<F: Fn(&char) -> bool>(&mut self, cursor: &mut usize, chars: &mut Vec<char>, predicate: F) -> Result<(), ConvertableBufferError> {
    let len = chars.len();

    // cleaning collectable_buffer
    self.collectable_buffer.clear();

    while *cursor < len {
      let char = chars.get(*cursor).unwrap();

      if predicate(char) {
        self.collectable_buffer.push(char.to_owned())?;
      } else {
        break
      }

      *cursor += 1;
    }

    Ok(())
  }

  #[inline(always)]
  fn collect_ident(&mut self, cursor: &mut usize, chars: &mut Vec<char>) -> Result<(), ConvertableBufferError> {
    self.collect_symbols(cursor, chars, |char| char.is_alphabetic())
  }

  #[inline(always)]
  fn collect_number(&mut self, cursor: &mut usize, chars: &mut Vec<char>) -> Result<(), ConvertableBufferError> {
    self.collect_symbols(cursor, chars, |char| char.is_digit(10))
  }
}

impl<'a> Lexer<'a> {
  #[inline(always)]
  pub fn buffer(&self) -> &Vec<Token> {
    &self.buffer
  }

  #[inline(always)]
  pub fn keywords(&self) -> &HashSet<&'a str> {
    &self.keywords
  }
}

#[derive(thiserror::Error, Debug)]
pub enum LexingError {
  #[error("Unknown token '{0}' ({1})")]
  UnknownToken(char, usize)
}