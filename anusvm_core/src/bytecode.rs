use anyhow::anyhow;
use crate::{AllocatedMemory, AnusVM, Function, object::Object, stack::AnusVMStackError::{self, MismatchedTypes}};

pub type ByteCode = Vec<ByteCodeInstruction>;

#[derive(Debug, Clone, PartialEq)]
pub enum ByteCodeInstruction {
  Add,
  AddF,
  Sub,
  SubF,
  Alloc,
  Push(Object),
  // todo remove String and just use stack.pop
  GetFunc(String),
  DefFunc(Function),
  Call,
  Pop
}

// ma favorite spaghetti
impl ByteCodeInstruction {
  // why do math operations returns -1 instead of 1?
  // imagine the stack at this moment
  // [Num1, Num2]
  // and i.g Add instruction pops two values
  // leaving the stack empty
  // but then it puts 1 value
  // and the stack size becomes 1
  // so basically -1 shows how much stack top has changed
  pub fn execute(&self, vm: &mut AnusVM) -> anyhow::Result<i32> {
    let stack = vm.stack();
    match self {
      Self::Add => self.impl_cmd_add(vm),
      Self::AddF => self.impl_cmd_addf(vm),
      Self::Sub => self.impl_cmd_sub(vm),
      Self::SubF => self.impl_cmd_subf(vm),
      Self::Alloc => self.impl_cmd_alloc(vm),
      Self::GetFunc(func_name) => Ok(vm.get_function(func_name)?),
      Self::DefFunc(func) => {
        vm.set_function(func.to_owned())?;
        Ok(0)
      },
      Self::Call => self.impl_cmd_call(vm),
      Self::Push(object) => {
        stack.push(object.to_owned())?;
        Ok(1)
      },
      Self::Pop => {
        stack.pop();
        Ok(0)
      },
    }
  }
}

// Internal command implementation
impl ByteCodeInstruction {
  #[inline]
  fn impl_cmd_add(&self, vm: &mut AnusVM) -> anyhow::Result<i32> {
    let stack = vm.stack();

    match (stack.pop(), stack.pop()) {
      (Object::Number(b), Object::Number(a)) => stack.push(Object::Number(a + b)),
      (Object::Number(_), value) => return Err(anyhow!(AnusVMStackError::MismatchedTypes(String::from("Number"), value.to_string()))),
      (value, Object::Number(_)) => return Err(anyhow!(AnusVMStackError::MismatchedTypes(String::from("Number"), value.to_string()))),
      _ => unreachable!()
    }?;

    Ok(-1)
  }

  fn impl_cmd_addf(&self, vm: &mut AnusVM) -> anyhow::Result<i32> {
    let stack = vm.stack();

    match (stack.pop(), stack.pop()) {
      (Object::Float(b), Object::Float(a)) => stack.push(Object::Float(a + b)),
      (Object::Float(_), value) => return Err(anyhow!(AnusVMStackError::MismatchedTypes(String::from("Float"), value.to_string()))),
      (value, Object::Float(_)) => return Err(anyhow!(AnusVMStackError::MismatchedTypes(String::from("Float"), value.to_string()))),
      _ => unreachable!()
    }?;

    Ok(-1)
  }

  fn impl_cmd_sub(&self, vm: &mut AnusVM) -> anyhow::Result<i32> {
    let stack = vm.stack();

    match (stack.pop(), stack.pop()) {
      (Object::Number(b), Object::Number(a)) => stack.push(Object::Number(a - b)),
      (Object::Number(_), value) => return Err(anyhow!(AnusVMStackError::MismatchedTypes(String::from("Number"), value.to_string()))),
      (value, Object::Number(_)) => return Err(anyhow!(AnusVMStackError::MismatchedTypes(String::from("Number"), value.to_string()))),
      _ => unreachable!()
    }?;

    Ok(-1)
  }

  fn impl_cmd_subf(&self, vm: &mut AnusVM) -> anyhow::Result<i32> {
    let stack = vm.stack();

    match (stack.pop(), stack.pop()) {
      (Object::Float(b), Object::Float(a)) => stack.push(Object::Float(a - b)),
      (Object::Float(_), value) => return Err(anyhow!(AnusVMStackError::MismatchedTypes(String::from("Float"), value.to_string()))),
      (value, Object::Float(_)) => return Err(anyhow!(AnusVMStackError::MismatchedTypes(String::from("Float"), value.to_string()))),
      _ => unreachable!()
    }?;

    Ok(-1)
  }

  fn impl_cmd_alloc(&self, vm: &mut AnusVM) -> anyhow::Result<i32> {
    let stack = vm.stack();

    let bytes = match stack.pop() {
      Object::Number(bytes) => bytes,
      value => return Err(anyhow!(AnusVMStackError::MismatchedTypes(String::from("Number"), value.to_string()))),
    };

    if bytes <= 0 {
      return Err(anyhow!(AnusVMStackError::AllocationFail(format!("an attempt to allocate {bytes} bytes failed"))));
    }

    let obj_result = AllocatedMemory::new(vm, bytes as usize);

    match obj_result {
      Ok(obj) => { vm.stack().push(Object::AllocatedMemory(obj)) },
      Err(err) => return Err(anyhow!(AnusVMStackError::AllocationFail(err.to_string())))
    }?;

    Ok(1)
  }

  fn impl_cmd_call(&self, vm: &mut AnusVM) -> anyhow::Result<i32> {
    let stack = vm.stack();

    let res = match stack.pop() {
      Object::Function(func) => func.call(vm)?,
      // NOT SAFE
      // absolutely unsafe !!!!!
      // TODO add some guarantee of ptr is valid
      Object::FunctionPtr(func_ptr) => unsafe { &(*func_ptr) }.call(vm)?,
      value => return Err(anyhow!(MismatchedTypes(String::from("Function"), value.to_string())))
    };

    Ok(res)
  }
}