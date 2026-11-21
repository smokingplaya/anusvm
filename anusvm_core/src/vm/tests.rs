#[test]
fn vm_check_namespacetree() {
  use crate::{AnusVM, vm::{namespace::{Namespace, NamespaceTreeValue}, object::Object}};

  let ns_std_drug = Namespace::try_from("std::drug".to_string())
    .unwrap();

  let mut vm = AnusVM::default();
  let namespace = vm.global_namespace_mut();
  namespace.insert(&ns_std_drug, Object::Number(228));

  assert!(namespace.resolve(&ns_std_drug) == Some(&NamespaceTreeValue::Object(Object::Number(228))));
}