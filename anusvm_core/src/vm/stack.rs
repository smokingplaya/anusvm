use crate::vm::object::Object;

pub const STACK_SIZE: usize = 1024;

#[derive(Debug)]
pub struct Stack {
  inner: [Object; STACK_SIZE]
}

impl Default for Stack {
  fn default() -> Self {
    Self {
      inner: [const { Object::Nil }; STACK_SIZE]
    }
  }
}