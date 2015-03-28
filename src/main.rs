extern crate rust_coverage_sample;

use rust_coverage_sample::*;

fn main() {
  //println!("{:?}", add(1i32, 5i32));
  let stud = rust_coverage_sample::stud::Student::new(10i32, 100u8);
  println!("Id : {:?}", stud.get_id());
}