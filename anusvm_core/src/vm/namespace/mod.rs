mod tests;
mod tree;

use std::ops::{Deref, DerefMut};

pub use tree::*;

// how much `::` you can use in namespace (i.g std::somename::somefn)
pub const NAMESPACE_SEPARATOR: &str = "::";

type NamespaceRaw = Vec<String>;

/// Namespace is a 
///
/// # Problem it solves
/// 
/// # Examples
/// `renderer::abstract::Backend`
/// where
/// - `renderer::abstract` is less significant part
/// - `Backend` is most significant part
#[derive(PartialEq)]
pub struct Namespace(NamespaceRaw);

// rules
impl Namespace {
  // validates part of Namespace
  // i.g part `std` of full namespace `std::somebusiness::KindaMonke`
  fn validate_name_single_part(part: &String) -> Result<(), NamespaceError> {
    match part.chars().enumerate().find(|(_, c)| !c.is_ascii()) {
      Some((pos, c)) => Err(NamespaceError::InvalidNamespaceCharacter(c, pos)),
      None => Ok(())
    }
  }
}

impl Namespace {
  pub unsafe fn from_raw(buffer: NamespaceRaw) -> Self {
    Self(buffer)
  }
}

impl TryFrom<&[String]> for Namespace {
  type Error = NamespaceError;

  fn try_from(buffer: &[String]) -> Result<Self, Self::Error> {
    // todo `std::math::awesome::` is prob valid :huh:
    buffer.into_iter()
      .try_for_each(Namespace::validate_name_single_part)?;

    let vec = buffer.iter().cloned().collect();

    Ok(Self(vec))
  }
}

impl TryFrom<String> for Namespace {
  type Error = NamespaceError;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    // genius
    let awkward = value.split(NAMESPACE_SEPARATOR);
    let buf = awkward.into_iter()
      .map(String::from).collect::<Vec<String>>();

    Self::try_from(&buf as &[String])
  }
}

// for native Iterators support and similar stuff
impl Deref for Namespace {
  type Target = NamespaceRaw;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for Namespace {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl ToString for Namespace {
  fn to_string(&self) -> String {
    self.join(NAMESPACE_SEPARATOR)
  }
}

#[derive(thiserror::Error, Debug)]
pub enum NamespaceError {
  #[error("invalid character '{0}' of namespace in position '{1}")]
  InvalidNamespaceCharacter(char, usize)
}