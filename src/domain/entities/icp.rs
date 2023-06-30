use chrono::{DateTime, Utc};

use crate::domain::types::icp_number::IcpNumber;

#[derive(Debug)]
pub struct Icp {
  number: IcpNumber,
  created_at: DateTime<Utc>,
}

impl Icp {
  pub fn new(number: String) -> Self {
    Icp {
      number: IcpNumber::new(number),
      created_at: Utc::now(),
    }
  }
}
