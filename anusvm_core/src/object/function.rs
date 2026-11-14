use crate::{AnusVM, ByteCode, stack::AnusVMStack};

pub type FunctionTypeCFunc = *const extern "C" fn(&mut AnusVMStack) -> i32;

#[derive(Debug, Clone, PartialEq)]
pub enum FunctionType {
  Bytecode(ByteCode),
  CFunc(FunctionTypeCFunc)
}

impl<'a> Default for FunctionType {
  fn default() -> Self {
    Self::Bytecode(Vec::with_capacity(16))
  }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct FunctionArguments {
  // todo
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Function {
  name: String,
  arguments: Option<FunctionArguments>,
  executable: FunctionType
}

// constructors
impl Function {
  // TODO validation of function name
  pub fn new(name: String, arguments: Option<FunctionArguments>, executable_bytecode: ByteCode) -> Self {
    Self { name, arguments, executable: FunctionType::Bytecode(executable_bytecode) }
  }

  pub fn new_from_cfunc(name: String, arguments: Option<FunctionArguments>, func: FunctionTypeCFunc) -> Result<Self, FunctionError> {
    if func.is_null() {
      return Err(FunctionError::InvalidFunctionPointer)
    }

    Ok(Self { name, arguments, executable: FunctionType::CFunc(func) })
  }
}

#[derive(thiserror::Error, Debug)]
pub enum FunctionError {
  #[error("InvalidFunctionPointer")]
  InvalidFunctionPointer,
  #[error("RuntimeError: {0}")]
  RuntimeError(String)
}

// getters
impl Function {
  pub fn name(&self) -> &String {
    &self.name
  }
}

impl Function {
  pub fn call(&self, vm: &mut AnusVM) -> Result<i32, FunctionError> {
    let stack = vm.stack();
    let len = stack.len();

    // the simpliest implementation of function calling
    // without registers and such cool things
    match &self.executable {
      FunctionType::Bytecode(bytecode) => {
        let res = vm.execute(&bytecode)
          .map_err(|e| FunctionError::RuntimeError(e.to_string()))?;

        Ok(len as i32 + res)
      },
      FunctionType::CFunc(cfuncptr) => {
        let stack = vm.stack();
        // safety: function contains validated pointer
        let res = unsafe { (**cfuncptr)(stack) };

        // todo error on stack handling
        Ok(len as i32 + res)
      }
    }
  }
}