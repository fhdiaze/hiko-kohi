#[derive(Debug)]
pub struct IcpNumber(String);

impl IcpNumber {
  pub fn new(number: String) -> Self {
    IcpNumber(number)
  }
}
