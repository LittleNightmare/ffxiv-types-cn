use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub struct UnknownVariant(pub &'static str, pub String);

impl Display for UnknownVariant {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    write!(f, "unknown variant {} for type {}", self.1, self.0)
  }
}
