use hiko_kohi::domain::entities::{state::State, time_line::TimeLine};

#[test]
pub fn new_time_line() {
  // Arrange

  // Act
  let time_line: TimeLine<i32> = TimeLine::new(vec![]);

  // Assert
  assert!(time_line.size() == 0);
}

#[test]
pub fn map_time_line() {
  // Arrange
  let values = vec![1, 2, 3];
  let states = values.into_iter().map(State::new).collect();
  let time_line: TimeLine<i32> = TimeLine::new(states);

  // Act
  let new_time_line = time_line.map(|x| x.to_string());

  // Assert
  assert!(time_line.size() == 3);
  assert!(new_time_line.size() == 3);
}
