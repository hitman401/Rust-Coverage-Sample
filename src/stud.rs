pub struct Student {
id: i32,
grade: u8
}

impl Student {
pub fn new(id: i32, grade: u8) -> Student {
  Student {
  id: id,
  grade: grade
  }
}

pub fn get_id(&self) -> i32 {
  self.id
}

pub fn get_grade(&self) -> u8 {
  self.grade
}
}

#[test]
fn test_student() {
  let stud = Student::new(10i32, 200u8);
  assert_eq!(10i32, stud.id);
}