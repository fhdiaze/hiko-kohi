use chrono::{DateTime, Utc};

use crate::domain::types::read_value::ReadValue;

pub struct Read {
  value: ReadValue,
  read_at: DateTime<Utc>,
}

pub enum ReadType {
  Hh,
  Customer,
  Contractor,
}
