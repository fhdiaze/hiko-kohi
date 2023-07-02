use chrono::{DateTime, Utc};

use crate::domain::types::read_value::ReadValue;

#[derive(Debug, PartialEq, Eq)]
pub struct Read {
  value: ReadValue,
  read_at: DateTime<Utc>,
}
