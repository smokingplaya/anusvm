mod namespace;
mod config;
mod object;
mod stack;

pub struct AnusVM {
  configuration: config::Configuration,
  stack: stack::Stack,
  namespace: namespace::NamespaceTree
}

fn main() {
  // let vm = AnusVM::new();

  // vm.insert(Namespace::from("std::somemodule::somestruct::Func")?, Object::Function(todo!()));
}