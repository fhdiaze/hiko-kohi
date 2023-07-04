use super::state::State;

#[derive(Debug)]
pub struct TimeLine<T> {
  states: Vec<State<T>>,
}

impl<T> TimeLine<T> {
  pub fn new(states: Vec<State<T>>) -> TimeLine<T> {
    TimeLine { states }
  }

  pub fn size(&self) -> usize {
    self.states.len()
  }

  pub fn map<F, U>(&self, f: F) -> TimeLine<U>
  where
    F: Fn(&T) -> U,
  {
    let new_states: Vec<State<U>> =
      self.states.iter().map(State::map(f)).collect();

    TimeLine { states: new_states }
  }
}
