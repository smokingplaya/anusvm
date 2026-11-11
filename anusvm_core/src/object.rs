use crate::bytecode::ByteCode;

// IMPORTANT
// if you modify Object, you also should modify ObjectType
// todo create Object and ObjectType in one macro

#[derive(Default, Debug, Clone, PartialEq)]
pub enum Object {
  #[default]
  Nil,
  Boolean(bool),
  Number(isize),
  Float(f32),
  String(String),
  Function(Function)
}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum ObjectType {
  #[default]
  Nil,
  Boolean,
  Number,
  Float,
  String,
  Function
}

impl Object {
  #[inline]
  pub fn r#type(&self) -> ObjectType {
    match self {
      Self::Nil => ObjectType::Nil,
      Self::Boolean(_) => ObjectType::Boolean,
      Self::Number(_) => ObjectType::Number,
      Self::Float(_) => ObjectType::Float,
      Self::String(_) => ObjectType::String,
      Self::Function(_) => ObjectType::Function,
    }
  }

  #[inline]
  pub fn is(&self, object_type: ObjectType) -> bool {
    self.r#type() == object_type
  }

  pub unsafe fn read_int(&self) -> isize {
    match self {
      Self::Number(isize) => *isize,
      _ => panic!("trying Object::read_int() on non `Number` Object")
    }
  }

  pub unsafe fn read_float(&self) -> f32 {
    match self {
      Self::Float(float) => *float,
      _ => panic!("trying Object::read_float() on non `Float` Object")
    }
  }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct FunctionArguments {
  // todo
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Function {
  name: String,
  arguments: FunctionArguments,
  instructions: ByteCode
}

impl Function {
  pub fn name(&self) -> &String {
    &self.name
  }
}