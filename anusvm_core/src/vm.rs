use crate::{bytecode::ByteCode, stack::AnusVMStack};

#[derive(Default, Debug, Clone)]
pub struct AnusVM {
  stack: AnusVMStack,
}

impl AnusVM {
  pub fn new() -> Self {
    Self::default()
  }
}

impl AnusVM {
  pub fn execute(&mut self, bytecode: &ByteCode) -> Result<i32, AnusVMError> {
    bytecode.iter()
      .try_fold(0, |acc, a| a.execute(self).map(|value| acc + value))
      .map_err(|e| AnusVMError::RuntimeError(e.to_string()))
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
  #[error("RuntimeError: {0}")]
  RuntimeError(String),
}