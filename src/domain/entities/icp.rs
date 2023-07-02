use chrono::{DateTime, Utc};

use crate::domain::types::icp_number::IcpNumber;

use super::read::Read;

#[derive(Debug, PartialEq, Eq)]
pub struct Icp {
  pub number: IcpNumber,
  pub created_at: DateTime<Utc>,
  pub reads: Vec<Read>,
}

impl Icp {
  pub fn new(number: String) -> Self {
    Icp {
      number: IcpNumber::new(number),
      created_at: Utc::now(),
      reads: Vec::new(),
    }
  }
}
