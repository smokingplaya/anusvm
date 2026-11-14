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
  // todo
  // (_, value)
  // value can be not number,
  // and
  pub fn execute(&self, vm: &mut AnusVM) -> anyhow::Result<i32> {
    // why do math operations returns -1 instead of 1?
    // imagine the stack at this moment
    // [Num1, Num2]
    // and i.g Add instruction pops two values
    // leaving the stack empty
    // but then it puts 1 value
    // and the stack size becomes 1
    // so basically -1 shows how much stack top has changed
    let stack = vm.stack();
    match self {
      Self::Add => {
        match (stack.pop(), stack.pop()) {
          (Object::Number(b), Object::Number(a)) => stack.push(Object::Number(a + b)),
          (Object::Number(_), value) => return Err(anyhow!(AnusVMStackError::MismatchedTypes(String::from("Number"), value.to_string()))),
          (value, Object::Number(_)) => return Err(anyhow!(AnusVMStackError::MismatchedTypes(String::from("Number"), value.to_string()))),
          _ => unreachable!()
        }?;

        Ok(-1)
      },
      Self::AddF => {
        match (stack.pop(), stack.pop()) {
          (Object::Float(b), Object::Float(a)) => stack.push(Object::Float(a + b)),
          (Object::Float(_), value) => return Err(anyhow!(AnusVMStackError::MismatchedTypes(String::from("Float"), value.to_string()))),
          (value, Object::Float(_)) => return Err(anyhow!(AnusVMStackError::MismatchedTypes(String::from("Float"), value.to_string()))),
          _ => unreachable!()
        }?;

        Ok(-1)
      },
      Self::Sub => {
        match (stack.pop(), stack.pop()) {
          (Object::Number(b), Object::Number(a)) => stack.push(Object::Number(a - b)),
          (Object::Number(_), value) => return Err(anyhow!(AnusVMStackError::MismatchedTypes(String::from("Number"), value.to_string()))),
          (value, Object::Number(_)) => return Err(anyhow!(AnusVMStackError::MismatchedTypes(String::from("Number"), value.to_string()))),
          _ => unreachable!()
        }?;

        Ok(-1)
      },
      Self::SubF => {
        match (stack.pop(), stack.pop()) {
          (Object::Float(b), Object::Float(a)) => stack.push(Object::Float(a - b)),
          (Object::Float(_), value) => return Err(anyhow!(AnusVMStackError::MismatchedTypes(String::from("Float"), value.to_string()))),
          (value, Object::Float(_)) => return Err(anyhow!(AnusVMStackError::MismatchedTypes(String::from("Float"), value.to_string()))),
          _ => unreachable!()
        }?;

        Ok(-1)
      },
      Self::Alloc => {
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
      },
      Self::GetFunc(func_name) => Ok(vm.get_function(func_name)?),
      Self::DefFunc(func) => {
        vm.set_function(func.to_owned())?;
        Ok(0)
      },
      Self::Call => {
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
      Self::Push(object) => {
        stack.push(object.to_owned())?;
        Ok(1)
      },
      Self::Pop => { stack.pop(); Ok(0) },
    }
  }
}