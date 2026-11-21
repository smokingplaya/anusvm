use crate::vm::object::Object;

pub const STACK_SIZE: usize = 1024;

pub struct Stack {
  inner: [Object; STACK_SIZE]
}