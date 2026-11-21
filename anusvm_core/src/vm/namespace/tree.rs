use crate::vm::object::Object;

pub struct NamespaceTree {
  // tree:
}

impl NamespaceTree {
  pub fn new() -> Self {
    Self {}
  }
}

impl NamespaceTree {
  pub fn resolve(namespace_buffer: &[&str]) -> Result<Object, NamespaceTreeError> {
    todo!()
  }

  pub fn insert(namespace_buffer: &[&str], object: Object) -> Result<(), NamespaceTreeError> {
    todo!()
  }

  pub fn erase(namespace_buffer: &[&str]) -> Result<Object, NamespaceTreeError> {
    todo!()
  }
}

#[derive()]
pub enum NamespaceTreeError {

}