use anyhow::{anyhow, bail};

use crate::{Function, object::{Object, ObjectType}, stack::{AnusVMStack, AnusVMStackError}};

pub type ByteCode = Vec<ByteCodeInstruction>;

#[derive(Debug, Clone, PartialEq)]
pub enum ByteCodeInstruction {
  Add,
  AddF,
  Sub,
  SubF,
  Push(Object),
  Pop
}

impl ByteCodeInstruction {
  pub fn execute(&self, stack: &mut AnusVMStack) -> anyhow::Result<()> {
    match self {
      Self::Add => {
        let a = stack.pop();
        let b = stack.pop();
        let r#type = b.r#type();

        let (a, b) = match (a, b) {
          (Object::Number(a), Object::Number(b)) => (a, b),
          _ => bail!(AnusVMStackError::MismatchedTypes(
            ObjectType::Number,
            r#type
          ))
        };

        // SAFETY: type check complete above
        stack.push(Object::Number(a + b))?;
      },
      Self::AddF => {
        let a = stack.pop_typechecked(ObjectType::Float)?;
        let b = stack.pop_typechecked(ObjectType::Float)?;
        // SAFETY: type check complete above
        stack.push(Object::Float(unsafe { a.read_float() + b.read_float() }))?;
      },
      Self::Sub => {
        let a = stack.pop_typechecked(ObjectType::Number)?;
        let b = stack.pop_typechecked(ObjectType::Number)?;
        // SAFETY: type check complete above
        stack.push(Object::Number(unsafe { a.read_int() - b.read_int() }))?;
      },
      Self::SubF => {
        let a = stack.pop_typechecked(ObjectType::Float)?;
        let b = stack.pop_typechecked(ObjectType::Float)?;
        // SAFETY: type check complete above
        stack.push(Object::Float(unsafe { a.read_float() - b.read_float() }))?;
      },
      Self::Push(object) => stack.push(object.to_owned())?,
      Self::Pop => { stack.pop(); },
    };

    Ok(())
  }
}