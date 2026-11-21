mod namespace;
mod config;
mod object;
mod stack;
mod tests;

#[derive(Default, Debug)]
pub struct AnusVM {
  configuration: config::Configuration,
  stack: stack::Stack,
  namespace: namespace::NamespaceTree
}

impl AnusVM {
  #[inline]
  pub fn global_namespace(&self) -> &namespace::NamespaceTree {
    &self.namespace
  }

  #[inline]
  pub fn global_namespace_mut(&mut self) -> &mut namespace::NamespaceTree {
    &mut self.namespace
  }
}