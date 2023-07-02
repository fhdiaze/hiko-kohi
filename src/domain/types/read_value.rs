#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadValue {
  dials: i64,
}

impl ReadValue {
  pub fn new(dials: i64) -> Self {
    ReadValue { dials }
  }
}
