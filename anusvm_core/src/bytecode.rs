use anyhow::anyhow;

use crate::{AllocatedMemory, AnusVM, object::Object, stack::AnusVMStackError::{self, MismatchedTypes}};

pub type ByteCode = Vec<ByteCodeInstruction>;

#[derive(Debug, Clone, PartialEq)]
pub enum ByteCodeInstruction {
  Add,
  AddF,
  Sub,
  SubF,
  Alloc(usize),
  Push(Object),
  Call,
  Pop
}

impl ByteCodeInstruction {
  // todo
  // (_, value)
  // value can be not number,
  // and
  pub fn execute(&self, vm: &mut AnusVM) -> anyhow::Result<i32> {
    // why do math operations returns -1 instead of 1?
    // imagine the stack at this moment
    // [Num1, Num2]
    // and i.g Add instruction pops two values
    // leaving the stack empty
    // but then it puts 1 value
    // and the stack size becomes 1
    // so basically -1 shows how much stack top has changed
    let stack = vm.stack();
    match self {
      Self::Add => {
        match (stack.pop(), stack.pop()) {
          (Object::Number(b), Object::Number(a)) => stack.push(Object::Number(a + b)),
          (Object::Number(_), value) => return Err(anyhow!(AnusVMStackError::MismatchedTypes(String::from("Number"), value.to_string()))),
          (value, Object::Number(_)) => return Err(anyhow!(AnusVMStackError::MismatchedTypes(String::from("Number"), value.to_string()))),
          _ => unreachable!()
        }?;

        Ok(-1)
      },
      Self::AddF => {
        match (stack.pop(), stack.pop()) {
          (Object::Float(b), Object::Float(a)) => stack.push(Object::Float(a + b)),
          (Object::Number(_), value) => return Err(anyhow!(AnusVMStackError::MismatchedTypes(String::from("Number"), value.to_string()))),
          (value, Object::Number(_)) => return Err(anyhow!(AnusVMStackError::MismatchedTypes(String::from("Number"), value.to_string()))),
          _ => unreachable!()
        }?;

        Ok(-1)
      },
      Self::Sub => {
        match (stack.pop(), stack.pop()) {
          (Object::Number(b), Object::Number(a)) => stack.push(Object::Number(a - b)),
          (Object::Number(_), value) => return Err(anyhow!(AnusVMStackError::MismatchedTypes(String::from("Number"), value.to_string()))),
          (value, Object::Number(_)) => return Err(anyhow!(AnusVMStackError::MismatchedTypes(String::from("Number"), value.to_string()))),
          _ => unreachable!()
        }?;

        Ok(-1)
      },
      Self::SubF => {
        match (stack.pop(), stack.pop()) {
          (Object::Float(b), Object::Float(a)) => stack.push(Object::Float(a - b)),
          (Object::Number(_), value) => return Err(anyhow!(AnusVMStackError::MismatchedTypes(String::from("Number"), value.to_string()))),
          (value, Object::Number(_)) => return Err(anyhow!(AnusVMStackError::MismatchedTypes(String::from("Number"), value.to_string()))),
          _ => unreachable!()
        }?;

        Ok(-1)
      },
      Self::Alloc(bytes) => {
        let obj_result = AllocatedMemory::new(vm, *bytes);

        match obj_result {
          Ok(obj) => { let stack = vm.stack(); stack.push(Object::AllocatedMemory(obj)) },
          Err(err) => return Err(anyhow!(AnusVMStackError::AllocationFail(err.to_string())))
        }?;

        Ok(1)
      },
      Self::Call => {
        let func = match stack.pop() {
          Object::Function(func) => func,
          value => return Err(anyhow!(MismatchedTypes(String::from("Function"), value.to_string())))
        };

        Ok(func.call(vm)?)
      }
      Self::Push(object) => {
        stack.push(object.to_owned())?;

        Ok(1)
      },
      Self::Pop => { stack.pop(); Ok(0) },
    }
  }
}