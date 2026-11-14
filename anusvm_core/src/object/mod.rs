mod function;
mod memory;

pub use function::*;
pub use memory::*;

#[derive(Default, Debug, Clone, PartialEq)]
pub enum Object {
  #[default]
  Nil,
  Boolean(bool),
  Number(isize),
  Float(f32),
  Function(Function),
  FunctionPtr(*const Function),
  AllocatedMemory(AllocatedMemory),
  Error
}

impl ToString for Object {
  fn to_string(&self) -> String {
    match self {
      Object::Nil => "Nil",
      Object::Boolean(_) => "Boolean",
      Object::Number(_) => "Number",
      Object::Float(_) => "Float",
      Object::Function(_) => "Function",
      Object::FunctionPtr(_) => "Function",
      Object::AllocatedMemory(_) => "AllocatedMemory",
      Object::Error => "Error",
    }.to_string()
  }
}