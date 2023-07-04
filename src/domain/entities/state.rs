use chrono::{DateTime, Utc};

#[allow(dead_code)]
type Fs<'a, T, U> = dyn Fn(&State<T>) -> State<U> + 'a;

#[derive(Debug)]
pub struct State<T> {
  value: T,
  created_at: DateTime<Utc>,
}

impl<T> State<T> {
  pub fn new(value: T) -> State<T> {
    State {
      value,
      created_at: Utc::now(),
    }
  }

  pub fn wrap(value: T, created_at: DateTime<Utc>) -> State<T> {
    State { value, created_at }
  }

  pub fn map<'a, F, U>(f: F) -> Box<Fs<'a, T, U>>
  where
    F: Fn(&T) -> U + 'a,
  {
    let m = move |x: &State<T>| State::new(f(&x.value));

    Box::new(m)
  }
}
