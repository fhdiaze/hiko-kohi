use hiko_kohi::domain::types::icp_number::IcpNumber;

#[test]
pub fn self_eq_icp_number() {
  // Arrange
  let icp_number_a = IcpNumber::new(String::from("ab"));

  // Act

  // Assert
  assert!(icp_number_a == icp_number_a)
}

#[test]
pub fn eq_icp_number() {
  // Arrange
  let icp_number_a = IcpNumber::new(String::from("ab"));
  let icp_number_b = IcpNumber::new(String::from("ab"));

  // Act

  // Assert
  assert!(icp_number_a == icp_number_b)
}

#[test]
pub fn not_eq_icp_number() {
  // Arrange
  let icp_number_a = IcpNumber::new(String::from(""));
  let icp_number_b = IcpNumber::new(String::from("abc"));

  // Act

  // Assert
  assert!(icp_number_a != icp_number_b)
}
