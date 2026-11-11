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
      keywords: HashSet::from(["add", "sub"]),
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
      let char = chars.get(cursor).unwrap()
        .to_owned();

      if char.is_whitespace() {
        cursor += 1;
        continue;
      }

      if char.is_digit(10) {
        self.collect_number(&mut cursor, &mut chars);
        // safety: right above
        self.buffer.push(Token::Number(unsafe { self.collectable_buffer_to_number() }));
      } else if char.is_alphabetic() {
        self.collect_ident(&mut cursor, &mut chars);

        let identifier = self.collectable_buffer_to_string();

        match self.keywords.contains(identifier.as_str()) {
          true => self.buffer.push(Token::Keyword(identifier)),
          false => self.buffer.push(Token::Identifier(identifier)),
        };
      }

      cursor += 1;
    }

    Ok(())
  }

  #[inline]
  fn collect_symbols<F: Fn(&char) -> bool>(&mut self, cursor: &mut usize, chars: &mut Vec<char>, predicate: F) {
    let len = chars.len();

    // cleaning collectable_buffer
    self.collectable_buffer_actual_len = 0;
    self.collectable_buffer.fill(0 as char);

    let mut buffer_index = 0;
    while *cursor < len {
      let char = chars.get(*cursor).unwrap();

      if predicate(char) {
        self.collectable_buffer[buffer_index] = char.to_owned();
        buffer_index += 1;
      } else {
        break
      }

      *cursor += 1;
    }

    self.collectable_buffer_actual_len = buffer_index;
  }

  // todo move collectable_buffer to custom struct
  #[inline(always)]
  fn collect_ident(&mut self, cursor: &mut usize, chars: &mut Vec<char>) {
    self.collect_symbols(cursor, chars, |char| char.is_alphabetic());
  }

  #[inline(always)]
  fn collect_number(&mut self, cursor: &mut usize, chars: &mut Vec<char>) {
    self.collect_symbols(cursor, chars, |char| char.is_digit(10));
  }

  fn collectable_buffer_to_string(&self) -> String {
    let buffer = &self.collectable_buffer[0..self.collectable_buffer_actual_len];
    let string = buffer.iter().collect::<String>();

    string
  }

  unsafe fn collectable_buffer_to_number(&self) -> i64 {
    self.collectable_buffer_to_string().parse()
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