use std::collections::HashMap;
use crate::vm::{namespace::Namespace, object::Object};

type NamespaceTreeType = HashMap<String, NamespaceTreeValue>;

#[derive(Default, Debug, PartialEq)]
pub struct NamespaceTree {
  inner: NamespaceTreeType
}

impl NamespaceTree {
  pub fn new() -> Self {
    Self {
      inner: HashMap::with_capacity(8)
    }
  }
}

// done crud
impl NamespaceTree {
  pub fn create_tree(&mut self, namespace: &Namespace) -> &mut NamespaceTreeType {
    let len = namespace.len();

    if len <= 1 {
      return &mut self.inner;
    }

    // less significant part
    // most significant is name of insertable Object
    let lsp = &namespace[0..len - 1];
    let mut tree_ref = self;

    // this loop is really sexpensive
    for part in lsp {
      tree_ref = match tree_ref.inner
        .entry(part.to_owned())
        .or_insert_with(|| NamespaceTreeValue::Tree(Self::new()))
      {
        NamespaceTreeValue::Tree(tree) => tree,
        NamespaceTreeValue::Object(_) => unreachable!()
      };
    }

    &mut tree_ref.inner
  }

  // create & update
  pub fn insert(&mut self, namespace: &Namespace, object: Object) -> Option<NamespaceTreeValue> {
    let tree = self.create_tree(namespace);

    tree.insert(
      namespace.last()?.to_owned(),
      NamespaceTreeValue::Object(object)
    )
  }

  // read
  pub fn resolve(&self, namespace: &Namespace) -> Option<&NamespaceTreeValue> {
    let len = namespace.len();

    if len == 0 {
      return None;
    }

    let mut tree_ref = self;

    for part in &namespace[..len - 1] {
      tree_ref = match tree_ref.inner.get(part)? {
        NamespaceTreeValue::Tree(tree) => tree,
        NamespaceTreeValue::Object(_) => return None
      };
    }

    tree_ref.inner.get(&namespace[len - 1])
  }

  // delete
  pub fn erase(&mut self, namespace: &Namespace) -> Option<NamespaceTreeValue> {
    let len = namespace.len();

    if len == 0 {
      return None;
    }

    let mut tree_ref = self;

    for part in &namespace[..len - 1] {
      tree_ref = match tree_ref.inner.get_mut(part)? {
        NamespaceTreeValue::Tree(tree) => tree,
        NamespaceTreeValue::Object(_) => return None
      };
    }

    tree_ref.inner.remove(&namespace[len - 1])
  }
}

#[derive(Debug, PartialEq)]
pub enum NamespaceTreeValue {
  Tree(NamespaceTree),
  Object(Object)
}