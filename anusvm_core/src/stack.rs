use crate::object::{Object, ObjectType};

const STACK_SIZE: usize = 1024;

pub struct AnusVMStackIndex(pub usize);

impl AnusVMStackIndex {
  pub fn new(index: usize) -> Result<Self, AnusVMStackIndexError> {
    if index > STACK_SIZE {
      return Err(AnusVMStackIndexError::InvalidIndex)
    }

    Ok(Self(index))
  }
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum AnusVMStackIndexError {
  #[error("InvalidIndex")]
  InvalidIndex
}

pub type AnusVMStackInner = [Option<Object>; STACK_SIZE];

#[derive(Debug, Clone)]
pub struct AnusVMStack {
  inner: AnusVMStackInner,
  len: usize
}

impl Default for AnusVMStack {
  fn default() -> Self {
    Self {
      // todo change to Object::Nil
      // it'll cost same memory
      // but idiomatically would be better
      inner: [const { None }; STACK_SIZE],
      len: 0
    }
  }
}

impl AnusVMStack {
  #[inline]
  pub fn len(&self) -> usize {
    self.len
  }

  pub fn pop(&mut self) -> Option<Object> {
    // cheap af
    let value = self.inner[self.len - 1].take();
    self.len -= 1;

    value
  }

  pub fn push(&mut self, object: Object) -> Result<(), AnusVMStackError> {
    if self.len + 1 >= self.inner.len() {
      return Err(AnusVMStackError::StackOverflow)
    }

    self.inner[self.len] = Some(object);
    self.len += 1;

    Ok(())
  }

  #[inline]
  /// Reads `self.inner` and cloning its value
  pub fn read(&self, index: AnusVMStackIndex) -> Option<Object> {
    let value = &self.inner[index.0];
    value.to_owned()
  }

  pub fn pop_typechecked(&mut self, object_type: ObjectType) -> Result<Object, AnusVMStackError> {
    let value = self.pop();
    let value_type = value.as_ref()
      .map_or(ObjectType::Nil, |value| value.r#type());

    if value_type != object_type {
      return Err(AnusVMStackError::MismatchedTypes(object_type, value_type))
    }

    if let Some(value) = value {
      return Ok(value)
    }

    unreachable!()
  }

  pub fn read_typechecked(&self, index: AnusVMStackIndex, object_type: ObjectType) -> Result<Object, AnusVMStackError> {
    let value = self.read(index);
    let value_type = value.as_ref()
      .map_or(ObjectType::Nil, |value| value.r#type());

    if value_type != object_type {
      return Err(AnusVMStackError::MismatchedTypes(object_type, value_type))
    }

    if let Some(value) = value {
      return Ok(value)
    }

    unreachable!()
  }

  pub fn snapshot(&self) -> &[Option<Object>] {
    &self.inner[0..self.len]
  }

  pub fn top(&self) -> &Option<Object> {
    &self.inner[self.len - 1]
  }
}

impl AnusVMStack {
  #[inline]
  pub fn stack_size(&self) -> usize {
    self.inner.len()
  }

  #[inline]
  pub fn stack_allocated_memory(&self) -> usize {
    self.inner.len() * std::mem::size_of::<Object>()
  }
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum AnusVMStackError {
  #[error("StackOverflow")]
  StackOverflow,
  #[error("MismatchedTypes: expected '{0:?}' received '{1:?}'")]
  MismatchedTypes(ObjectType, ObjectType)
}