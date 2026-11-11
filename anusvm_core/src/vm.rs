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
  pub fn execute(&mut self, bytecode: &ByteCode) -> Result<(), AnusVMError> {
    bytecode.iter()
      .try_for_each(|a| a.execute(&mut self.stack))
      .map_err(|e| return AnusVMError::InvalidByteCode(e.to_string()))
  }
}

impl AnusVM {
  #[inline]
  pub fn stack(&mut self) -> &mut AnusVMStack {
    &mut self.stack
  }
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum AnusVMError {
  #[error("InvalidByteCode: {0}")]
  InvalidByteCode(String),
}