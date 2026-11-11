use crate::{bytecode::ByteCode, stack::AnusVMStack};

#[derive(Default, Debug, Clone)]
pub struct AnusVM {
  stack: AnusVMStack
}

impl AnusVM {
  pub fn new() -> Self {
    Self::default()
  }
}

impl AnusVM {
  pub fn execute(bytecode: &ByteCode) -> Result<(), AnusVMError> {
    todo!()
  }
}

impl AnusVM {
  #[inline]
  pub fn stack_size(&self) -> usize {
    self.stack.stack_size()
  }

  #[inline]
  pub fn stack_allocated_memory(&self) -> usize {
    self.stack.stack_allocated_memory()
  }
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum AnusVMError {
  #[error("BrokenByteCode")]
  BrokenByteCode,
}