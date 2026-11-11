use std::io::{BufRead, stdin, stdout, Write};
use anusvm_core::AnusVirtualMachine;
use crate::{lexer::{Lexer, Token}, parser::Parser};

mod lexer;
mod parser;

// anusVM™ REPL
fn main() -> anyhow::Result<()> {
  let _vm = AnusVirtualMachine::new();
  let mut lexer = Lexer::new();
  let _parser = Parser::new();

  println!(
    "anus Virtual Machine™ REPL Interactive environment v{} ({}-{})\nsmokingplaya 2026 (redistributed under {})",
    env!("CARGO_PKG_VERSION"),
    env!("GIT_COMMIT_SHORT"),
    env!("GIT_BRANCH"),
    env!("CARGO_PKG_LICENSE")
  );

  let stdin = stdin();
  let mut stdout = stdout();
  loop {
    print!("& ");
    stdout.flush()?;

    let mut line_buffer = String::with_capacity(16);

    if stdin.lock().read_line(&mut line_buffer)? == 0 {
      println!();
      break;
    }

    let line = line_buffer.trim_end();

    if let Err(err) = lexer.tokenize(line.to_string()) {
      println!("Syntax error: {err}");
    }

    let token_buffer = lexer.buffer();

    if let Some(command) = token_buffer.get(0) {
      if command.to_owned() == Token::Identifier("quit".to_string()) {
        break
      }
    }

    dbg!(&token_buffer);
  }

  Ok(())
}