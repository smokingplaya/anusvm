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

#[test]
fn vm_check_root_namespace() {
  use crate::{AnusVM, vm::{namespace::{Namespace, NamespaceTreeValue}, object::Object}};

  let mut vm = AnusVM::default();
  let namespace = vm.global_namespace_mut();

  let ns_root_number = Namespace::try_from("rootnamespace_number".to_string())
    .unwrap();

  namespace.insert(&ns_root_number, Object::Number(4099));

  assert!(namespace.resolve(&ns_root_number) == Some(&NamespaceTreeValue::Object(Object::Number(4099))));
}

#[test]
fn check_invalid_namespace() {
  use crate::{AnusVM, vm::{namespace::{Namespace, NamespaceTreeValue}, object::Object}};

  let mut vm = AnusVM::default();
  let namespace = vm.global_namespace_mut();

  assert!(Namespace::try_from("std::wrong::namespace::".to_string()).is_err());
}