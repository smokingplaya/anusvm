use crate::stack::AnusVMStack;

pub type ByteCode = Vec<ByteCodeInstruction>;

#[derive(Debug, Clone)]
pub enum ByteCodeInstruction {
  Add,
  Sub,
}

impl ByteCodeInstruction {
  pub fn execute(&self, stack: &mut AnusVMStack) {
    match self {
      Self::Add => {
        stack.pop();
      },
      Self::Sub => {}
    }
  }
}