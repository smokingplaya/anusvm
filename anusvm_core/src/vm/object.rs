#[derive(Default, Debug, PartialEq)]
pub enum Object {
  #[default]
  Nil,
  Number(isize)
}