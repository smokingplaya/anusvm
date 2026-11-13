mod function;

pub use function::*;

// IMPORTANT
// if you modify Object, you also should modify ObjectType
// todo create Object and ObjectType in one macro

#[derive(Default, Debug, Clone, PartialEq)]
pub enum Object {
  #[default]
  Nil,
  Boolean(bool),
  Number(isize),
  Float(f32),
  String(String),
  Function(Function),
  Error
}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum ObjectType {
  #[default]
  Nil,
  Boolean,
  Number,
  Float,
  String,
  Function,
  Error
}

impl Object {
  #[inline]
  pub fn r#type(&self) -> ObjectType {
    match self {
      Self::Nil => ObjectType::Nil,
      Self::Boolean(_) => ObjectType::Boolean,
      Self::Number(_) => ObjectType::Number,
      Self::Float(_) => ObjectType::Float,
      Self::String(_) => ObjectType::String,
      Self::Function(_) => ObjectType::Function,
      Self::Error => ObjectType::Error
    }
  }

  #[inline]
  pub fn is(&self, object_type: ObjectType) -> bool {
    self.r#type() == object_type
  }

  #[inline]
  pub unsafe fn read_int(&self) -> isize {
    match self {
      Self::Number(isize) => *isize,
      _ => panic!("trying Object::read_int() on non `Number` Object")
    }
  }

  #[inline]
  pub unsafe fn read_float(&self) -> f32 {
    match self {
      Self::Float(float) => *float,
      _ => panic!("trying Object::read_float() on non `Float` Object")
    }
  }

  #[inline]
  pub unsafe fn read_func(&self) -> &Function {
    match self {
      Self::Function(func) => func,
      _ => panic!("trying Object::read_func() on non `Function` Object")
    }
  }
}