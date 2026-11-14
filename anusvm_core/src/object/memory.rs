use anyhow::Result;
use crate::AnusVM;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct AllocatedMemory {
  // why vec?
  // fast, convient, automatically memory reallocation when needed, Drop already implemented
  memory: Vec<u8>,
}

impl AllocatedMemory {
  pub fn new(_vm: &mut AnusVM, len: usize) -> Result<Self> {
    Ok(Self { memory: Vec::with_capacity(len) })
  }
}

impl AllocatedMemory {
  #[inline]
  pub fn len(&self) -> usize {
    self.memory.len()
  }

  #[inline]
  pub fn read<'a>(&'a self, offset: usize, len: usize) -> &'a [u8] {
    &self.memory[offset..len]
  }

  // TODO: wrong offset/data.len threatens to break struct layout
  pub fn write<'a>(&'a mut self, offset: usize, data: &[u8]) -> Result<(), AllocatedMemoryError> {
    if offset + data.len() > self.len() {
      return Err(AllocatedMemoryError::OutOfBounds);
    }

    self.memory[offset..offset+data.len()].copy_from_slice(&data[0..data.len()]);

    Ok(())
  }
}

#[derive(thiserror::Error, Debug)]
pub enum AllocatedMemoryError {
  #[error("OutOfBounds")]
  OutOfBounds
}