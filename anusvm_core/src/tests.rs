#[test]
fn number_addition() -> anyhow::Result<()> {
  use std::time::Instant;
  use crate::{AnusVM, ByteCodeInstruction, Object};

  let mut vm = AnusVM::new();

  {
    let stack = vm.stack();
    stack.push(Object::Number(2))?;
    stack.push(Object::Number(4))?;
  }

  let test_instructions: Vec<ByteCodeInstruction> = vec![
    ByteCodeInstruction::Add
  ];

  let instant = Instant::now();

  vm.execute(&test_instructions)?;

  println!("AnusVM::execute elapsed {}μs", instant.elapsed().as_micros());

  let stack = vm.stack();

  assert!(stack.pop() == Object::Number(6));

  Ok(())
}