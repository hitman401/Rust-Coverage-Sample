fn add(a: i32, b: i32) -> isize {
	(a + b) as isize
}

fn main() {
	println!("{:?}", add(1i32, 3i32));
}

#[test]
fn add_test() {
	assert_eq!(10isize, add(4, 6))
}