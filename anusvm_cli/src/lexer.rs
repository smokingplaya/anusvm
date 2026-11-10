use std::collections::HashSet;

const BIT_WIDTH: usize = if cfg!(target_pointer_width = "64") { 64 } else { 32 };

/// Lexem implementation
#[derive(Debug, Clone)]
pub enum Token {
  Keyword(String),
  Identifier(String),
  Number(i64)
  // Assign, // =
  // LeftParenthesis, // (
  // RightParenthesis, // )
  // LeftBracket, // [
  // RightBracket, // ]
  // LeftBrace, // {
  // RightBrace, // }
  // Plus, // +
  // Minus, // -
  // Multiply, // *
  // Divide, // /
  // Equal, // ==
  // NotEqual, // !=
  // Less, // <
  // Greater, // >
  // LessEqual, // <=
  // GreaterEqual, // >=
  // Comma, // ,
  // Dot, // .
  // Semicolon, // ;
  // Colon // :
}

// allocation hater
#[derive(Debug)]
pub struct Lexer<'a> {
  keywords: HashSet<&'a str>,
  buffer: Vec<Token>,
  collectable_buffer_actual_len: usize,
  collectable_buffer: [char; BIT_WIDTH]
}

/// Constructor
impl<'a> Lexer<'a> {
  #[inline(always)]
  pub fn new() -> Self {
    Self {
      keywords: HashSet::from(["add", "sub"]), // SYSCALL ENTERTAINMENT
      buffer: Vec::with_capacity(16),
      collectable_buffer_actual_len: 0,
      collectable_buffer: [0 as char; BIT_WIDTH]
    }
  }
}

impl<'a> Lexer<'a> {
  pub fn tokenize(&mut self, input: String) -> Result<(), LexingError> {
    let mut cursor = 0;
    let len = input.len();

    self.buffer.clear();

    if len == 0 {
      return Ok(())
    }

    let mut chars = input.chars().collect::<Vec<char>>();
    while cursor < len {
      // nth mutates `chars` and removes element on `cursor` position
      let char = chars.get(cursor).unwrap();

      if char.is_whitespace() {
        cursor += 1;
        continue;
      }

      if char.is_digit(10) {
        self.collect_number(&mut cursor, &mut chars, len);
        self.buffer.push(Token::Number(self.convert_buffer_to_number()));
      }

      // todo
      // if char.is_alphabetic() {
      // }

      cursor += 1;
    }

    Ok(())
  }

  #[inline(always)]
  fn collect_number(&mut self, cursor: &mut usize, chars: &mut Vec<char>, len: usize) {
    self.collectable_buffer_actual_len = 0;
    self.collectable_buffer.fill(0 as char);

    let mut buffer_index = 0;
    while *cursor < len {
      let char = chars.get(*cursor).unwrap();

      if char.is_digit(10) {
        self.collectable_buffer[buffer_index] = char.to_owned();
        buffer_index += 1;
      } else {
        break
      }

      *cursor += 1;
    }

    self.collectable_buffer_actual_len = buffer_index;
  }

  fn convert_buffer_to_number(&self) -> i64 {
    let buffer = &self.collectable_buffer[0..self.collectable_buffer_actual_len];
    let string = buffer.iter().collect::<String>();

    string.parse()
      .unwrap()
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
  #[error("InvalidInput")]
  InvalidInput
}