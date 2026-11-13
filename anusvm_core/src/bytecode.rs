use anyhow::bail;
use crate::{AnusVM, object::{Object, ObjectType}, stack::AnusVMStackError};

pub type ByteCode = Vec<ByteCodeInstruction>;

#[derive(Debug, Clone, PartialEq)]
pub enum ByteCodeInstruction {
  Add,
  AddF,
  Sub,
  SubF,
  Push(Object),
  Call,
  Pop
}

impl ByteCodeInstruction {
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
        let b = stack.pop_typechecked(ObjectType::Number)?;
        let a = stack.pop_typechecked(ObjectType::Number)?;

        // SAFETY: type check completed above
        stack.push(Object::Number(unsafe { a.read_int() + b.read_int() }))?;

        Ok::<_,anyhow::Error>(-1)
      },
      Self::AddF => {
        let b = stack.pop_typechecked(ObjectType::Float)?;
        let a = stack.pop_typechecked(ObjectType::Float)?;
        // SAFETY: type check completed above
        stack.push(Object::Float(unsafe { a.read_float() + b.read_float() }))?;

        Ok(-1)
      },
      Self::Sub => {
        let b = stack.pop_typechecked(ObjectType::Number)?;
        let a = stack.pop_typechecked(ObjectType::Number)?;
        // SAFETY: type check completed above
        stack.push(Object::Number(unsafe { a.read_int() - b.read_int() }))?;

        Ok(-1)
      },
      Self::SubF => {
        let b = stack.pop_typechecked(ObjectType::Float)?;
        let a = stack.pop_typechecked(ObjectType::Float)?;
        // SAFETY: type check completed above
        stack.push(Object::Float(unsafe { a.read_float() - b.read_float() }))?;

        Ok(-1)
      },
      Self::Call => {
        let func_obj = stack.pop_typechecked(ObjectType::Function)?;
        // SAFETY: type check complete above
        let func = unsafe { func_obj.read_func() };
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