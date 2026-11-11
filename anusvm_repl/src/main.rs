use std::io::{BufRead, Write, stdin, stdout};
use anusvm_core::{AnusVM, ByteCode, ByteCodeInstruction, Object};
use crate::{lexer::{Lexer, Token}, parser::Parser};

mod lexer;
mod parser;

// anusVM™ REPL
fn main() -> anyhow::Result<()> {
  println!("{}\n{}\n{}\n{}\n{}",
    "anus Virtual Machine™ REPL",
    format!(
      "interactive environment version {} ({}-{})",
      env!("CARGO_PKG_VERSION"),
      env!("CARGO_PKG_VERSION"),
      env!("GIT_COMMIT_SHORT"),
    ),
    format!("smokingplaya 2026 (redistributed under the {} license)", env!("CARGO_PKG_LICENSE")),
    format!("{} profile built on {}", env!("BUILDDATA_PROFILE"), env!("BUILDDATA_USER")),
    format!("build at {}", env!("BUILDDATA_TIME"))
  );

  let mut vm = AnusVM::new();
  let mut lexer = Lexer::new();
  let _parser = Parser::new();
  let stack = vm.stack();

  println!("stackSize = {}, allocated memory = {}KiB", stack.stack_size(), stack.stack_allocated_memory() / 1024);

  let stdin = stdin();
  let mut stdout = stdout();
  loop {
    print!("> ");
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
      if command.to_owned() == Token::Identifier("exit".to_string()) {
        break
      }
    }

    dbg!(&token_buffer);
  }

  Ok(())
}