use crate::{Function, object::{Object, ObjectType}, stack::AnusVMStack};

pub type ByteCode = Vec<ByteCodeInstruction>;

#[derive(Debug, Clone, PartialEq)]
pub enum ByteCodeInstruction {
  Add,
  AddF,
  Sub,
  SubF,
  PushNil,
  PushBool(bool),
  PushNum(isize),
  PushFt(f32),
  PushStr(String),
  PushFn(Function),
  Pop
}

impl ByteCodeInstruction {
  pub fn execute(&self, stack: &mut AnusVMStack) -> anyhow::Result<()> {
    match self {
      Self::Add => {
        let a = stack.pop_typechecked(ObjectType::Number)?;
        let b = stack.pop_typechecked(ObjectType::Number)?;
        // SAFETY: type check complete above
        stack.push(Object::Number(unsafe { a.read_int() + b.read_int() }))?;
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
      Self::PushNil => stack.push(Object::Nil)?,
      Self::PushBool(bool) => stack.push(Object::Boolean(*bool))?,
      Self::PushNum(num) => stack.push(Object::Number(*num))?,
      Self::PushFt(ft) => stack.push(Object::Float(*ft))?,
      Self::PushStr(str) => stack.push(Object::String(str.clone()))?,
      Self::PushFn(func) => stack.push(Object::Function(func.clone()))?,
      Self::Pop => { stack.pop(); },
    };

    Ok(())
  }
}