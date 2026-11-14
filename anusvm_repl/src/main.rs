use anusvm_core::{AnusVM, Object};
use std::{io::{BufRead, Write, stdin, stdout}, time::Instant};
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
  let mut parser = Parser::new();

  debug_stack_info(&mut vm);

  let stdin = stdin();
  let mut stdout = stdout();

  let mut bytecode_buffer = Vec::with_capacity(64);

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

    if let Some(command) = token_buffer.first() {
      if *command == Token::Identifier("exit".to_string()) {
        break
      }
    }

    // dbg!(&token_buffer);

    if let Err(err) = parser.parserize(token_buffer, &mut bytecode_buffer, None) {
      println!("{err}");
      continue
    };

    let instant = Instant::now();

    if let Err(err) = vm.execute(&bytecode_buffer) {
      println!("{err}");
    }

    println!("vm.execute elapsed {}ns", instant.elapsed().as_nanos());

    bytecode_buffer.clear();

    let object = vm.stack().top();

    let to_print = match object {
      Object::Nil => continue,
      Object::Boolean(bool) => &bool.to_string(),
      Object::Function(func) => &format!("Function {}", func.name()),
      Object::FunctionPtr(func) => &format!("Function {}", unsafe { &(*(*func)) }.name()),
      Object::Number(number) => &number.to_string(),
      Object::Float(float) => &float.to_string(),
      Object::AllocatedMemory(memory) => &format!("Allocated {} bytes {:p}", memory.len(), memory),
      Object::Error => &"unknown error".to_string()
    };

    println!("$ {to_print}");
  }


  Ok(())
}

fn debug_stack_info(vm: &mut AnusVM) {
  let stack = vm.stack();
  println!("stack size = {}, allocated stack memory = {}KiB", stack.stack_size(), stack.stack_allocated_memory() / 1024);
}