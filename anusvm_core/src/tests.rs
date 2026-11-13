use crate::stack::AnusVMStack;

#[test]
fn number_addition() -> anyhow::Result<()> {
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

  vm.execute(&test_instructions)?;

  let stack = vm.stack();

  assert!(stack.pop() == Object::Number(6));

  Ok(())
}

#[test]
fn test_bytecode_function() -> anyhow::Result<()> {
  use crate::{object::Function, AnusVM, ByteCodeInstruction, Object, FunctionArguments, ObjectType};

  let mut vm = AnusVM::new();

  // pushing function on stack
  {
    let stack = vm.stack();

    let bytecode = vec! [
      ByteCodeInstruction::Add,
      // 194'll be on stack
      ByteCodeInstruction::Push(Object::Number(666)),
      ByteCodeInstruction::Add, // 194 + 666
      ByteCodeInstruction::Push(Object::Number(1)),
      ByteCodeInstruction::Sub // 860 - 1
    ];

    // currently function.name is unused
    // and function doesn't stored anywhere so
    let func = Function::new("test".to_string(), FunctionArguments {}, bytecode);
    stack.push(Object::Function(func))?;
  }

  let func_obj;
  let func;
  {
    let stack = vm.stack();

    func_obj = stack.pop_typechecked(ObjectType::Function)?;
    func = unsafe { func_obj.read_func() };

    stack.push(Object::Number(66))?;
    stack.push(Object::Number(128))?;
  }

  let result;
  {
    result = func.call(&mut vm)?;
  }

  let stack = vm.stack();

  assert!(result == 1);
  assert!(stack.pop() == Object::Number(859));

  Ok(())
}

#[test]
fn test_cfunc() -> anyhow::Result<()> {
  use crate::{object::Function, AnusVM, Object, FunctionArguments, ObjectType};

  let mut vm = AnusVM::new();

  // pushing function on stack
  {
    let stack = vm.stack();

    let func: extern "C" fn(&mut AnusVMStack) -> i32 = non_test_cfunction;
    // currently function.name is unused
    // and function doesn't stored anywhere so
    let func = Function::new_from_cfunc("test".to_string(), FunctionArguments {}, &func)?;
    stack.push(Object::Function(func))?;
  }

  let func_obj;
  let func;
  {
    let stack = vm.stack();

    func_obj = stack.pop_typechecked(ObjectType::Function)?;
    func = unsafe { func_obj.read_func() };

    stack.push(Object::Number(66))?;
    stack.push(Object::Number(128))?;
  }

  let result;
  {
    result = func.call(&mut vm)?;
  }

  let stack = vm.stack();

  assert!(result == 3);
  assert!(stack.pop() == Object::Number(968433));

  Ok(())
}

#[allow(unused)]
extern "C" fn non_test_cfunction(stack: &mut AnusVMStack) -> i32 {
  stack.push(crate::Object::Number(968433)).is_ok() as i32 // genius
}