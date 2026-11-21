#[test]
fn validate_namespaces() {
  use crate::vm::namespace::Namespace;

  let table: [(&str, Vec<&str>); 3] = [
    ("std::math::awesome", vec!["std", "math", "awesome"]),
    ("std", vec!["std"]),
    ("gameengine::deepsea::Instance", vec!["gameengine", "deepsea", "Instance"]),
  ];

  table.iter().for_each(|(namespace_raw, validation_namespace)| {
    let namespace = Namespace::try_from(namespace_raw.to_string()).unwrap();
    let validation_namespace = validation_namespace.iter()
      .map(|ass| ass.to_string())
      .collect::<Vec<String>>();

    assert!(namespace == unsafe { Namespace::from_raw(validation_namespace) })
  });
}