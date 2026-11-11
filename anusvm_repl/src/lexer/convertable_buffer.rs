const BUFFER_LEN: usize = if cfg!(target_pointer_width = "64") { 64 } else { 32 };

#[derive(Debug, Clone)]
pub struct ConvertableBuffer {
  inner: [char; BUFFER_LEN],
  len: usize,
}

impl ConvertableBuffer {
  #[inline(always)]
  pub fn new() -> Self {
    Self {
      inner: [0 as char; BUFFER_LEN],
      len: 0
    }
  }
}

impl ConvertableBuffer {
  pub fn push(&mut self, char: char) -> Result<(), ConvertableBufferError> {
    if self.len >= BUFFER_LEN {
      return Err(ConvertableBufferError::BufferOverflow)
    }

    self.inner[self.len] = char;
    self.len += 1;

    Ok(())
  }

  pub fn clear(&mut self) {
    self.inner.fill(0 as char);
    self.len = 0;
  }
}

impl ConvertableBuffer {
  pub unsafe fn convert<T: Convertable>(&self) -> T {
    let actual_slice = &self.inner[0..self.len];
    let string = actual_slice.iter()
      .collect::<String>();

    unsafe { T::convert(string) }
  }
}

pub trait Convertable {
  unsafe fn convert(string: String) -> Self;
}

impl Convertable for String {
  #[inline]
  unsafe fn convert(string: String) -> Self {
    string
  }
}

macro_rules! declare_converable_num {
  ($t:ty) => {
    impl Convertable for $t {
      #[inline]
      unsafe fn convert(string: String) -> $t {
        string.parse::<$t>()
          .unwrap()
      }
    }
  };
}

declare_converable_num!(u8);
declare_converable_num!(u16);
declare_converable_num!(u32);
declare_converable_num!(u64);
declare_converable_num!(u128);
declare_converable_num!(usize);

declare_converable_num!(i8);
declare_converable_num!(i16);
declare_converable_num!(i32);
declare_converable_num!(i64);
declare_converable_num!(i128);
declare_converable_num!(isize);

declare_converable_num!(f32);
declare_converable_num!(f64);


#[derive(thiserror::Error, Debug, Clone)]
pub enum ConvertableBufferError {
  #[error("BufferOverflow")]
  BufferOverflow
}