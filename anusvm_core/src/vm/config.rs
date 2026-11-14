#[derive(Debug, Clone)]
pub struct AnusVMConfiguration {
  pub allow_override_functions: bool
}

impl Default for AnusVMConfiguration {
  fn default() -> Self {
    Self {
      allow_override_functions: false
    }
  }
}