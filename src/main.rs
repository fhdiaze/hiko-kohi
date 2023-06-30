use crate::domain::entities::icp::Icp;

mod domain;

fn main() {
  println!("Hello, world!");
  let icp = Icp::new(String::from("number"));

  println!("{:#?}", icp);
}
