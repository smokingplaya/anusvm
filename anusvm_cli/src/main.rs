use std::io::{BufRead, stdin, stdout, Write};
use anusvm_core::AnusVirtualMachine;
use crate::{lexer::Lexer, parser::Parser};

mod lexer;
mod parser;

// anusVM™
fn main() -> anyhow::Result<()> {
  let vm = AnusVirtualMachine::new();
  let mut lexer = Lexer::new();
  let parser = Parser::new();

  println!("anus Virtual Machine™\n{} ({} {}) by smokingplaya<3", env!("CARGO_PKG_VERSION"), env!("GIT_COMMIT_SHORT"), env!("GIT_BRANCH"));

  print!("> ");
  stdout().flush()?;

  let stdin = stdin();
  for line in stdin.lock().lines() {
    print!("> ");
    stdout().flush()?;

    let line = line.unwrap();

    match lexer.tokenize(line) {
      Ok(_) => {},
      Err(err) => {
        println!("SyntaxFail: {err}");
        continue;
      }
    };

    let token_buffer = lexer.buffer();
    dbg!(token_buffer);
    // parser.parserize(token_buffer)?;

    print!("\r");
  }

  Ok(())
}