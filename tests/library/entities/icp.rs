use hiko_kohi::domain::entities::icp::Icp;

#[test]
pub fn new_icp() {
  // Arrange

  // Act
  let icp = Icp::new("abcd".to_string());

  // Assert
  assert!(icp.reads.is_empty());
}

#[test]
pub fn self_eq_icp() {
  // Arrange

  // Act
  let icp = Icp::new("abcd".to_string());

  // Assert
  assert!(icp == icp);
}
