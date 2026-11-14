use anyhow::Result;
use crate::AnusVM;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct AllocatedMemory {
  // why vec?
  // fast, convient, automatically memory reallocation when needed, Drop already implemented
  memory: Vec<u8>,
}

impl AllocatedMemory {
  pub fn new(vm: &mut AnusVM, len: usize) -> Result<Self> {
    Ok(Self { memory: Vec::with_capacity(len) })
  }
}

impl AllocatedMemory {
  #[inline]
  pub fn len(&self) -> usize {
    self.memory.len()
  }

  #[inline]
  pub fn read<'a>(&'a self, offset: usize, count: usize) -> &'a [u8] {
    &self.memory[offset..count]
  }
}