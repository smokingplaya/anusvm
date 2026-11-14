use std::collections::HashMap;
use crate::{Function, Object, bytecode::ByteCode, stack::AnusVMStack};

mod config;

pub use config::AnusVMConfiguration;

#[derive(Default, Debug, Clone)]
pub struct AnusVM {
  stack: AnusVMStack,
  config: AnusVMConfiguration,
  func_storage: HashMap<String, Function>,
}

impl AnusVM {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn with_config(config: AnusVMConfiguration) -> Self {
    Self {
      config,
      func_storage: HashMap::with_capacity(64),
      ..Default::default()
    }
  }
}

impl AnusVM {
  pub fn execute(&mut self, bytecode: &ByteCode) -> Result<i32, AnusVMError> {
    bytecode.iter()
      .try_fold(0, |acc, a| a.execute(self).map(|value| acc + value))
      .map_err(|e| AnusVMError::RuntimeError(e.to_string()))
      // .map_err(|e| AnusVMError::RuntimeError(e.source().unwrap().to_string()))
  }
}

impl AnusVM {
  #[inline]
  pub fn stack(&mut self) -> &mut AnusVMStack {
    &mut self.stack
  }
}

impl AnusVM {
  pub fn set_function(&mut self, func: Function) -> Result<(), AnusVMError> {
    let func_name = func.name();

    if self.func_storage.contains_key(func_name)
      && !self.config.allow_override_functions {
      return Err(AnusVMError::RuntimeError(format!("function '{}' already defined", func_name)))
    }

    self.func_storage.insert(func_name.to_owned(), func);

    Ok(())
  }

  pub fn get_function(&mut self, name: &String) -> Result<i32, AnusVMError> {
    match self.func_storage.get(name) {
      Some(func) => self.stack.push(Object::FunctionPtr(func))
        .map_err(|e| AnusVMError::RuntimeError(e.to_string()))?,
      None => return Err(AnusVMError::RuntimeError(format!("function '{name}' not found")))
    };

    Ok(1)
  }
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum AnusVMError {
  #[error("RuntimeError: {0}")]
  RuntimeError(String),
}