pub mod stud;
pub mod faculty;

pub fn add(a: i32, b: i32) -> isize {
	(a + b) as isize
}

pub fn sub(a: i32, b: i32) -> isize {
	(a + b) as isize
}

fn add_test() {
	assert_eq!(10isize, add(4, 6));
}

#[test]
fn test() {
	add_test();
}
