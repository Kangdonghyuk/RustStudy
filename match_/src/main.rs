fn plus_one(x: Option<i32>) -> Option<i32> {
	match x {
		None => None,
		Some(i) => Some(i+1),
	}
}

fn main() {

	let five = Some(5);
	let six = plus_one(five);
	let none = plus_one(None);

	println!("{:?}", five);
	println!("{:?}", six);

	let mut count = 0;

	let some_u8_value = Some(3);

	if let Some(3) = some_u8_value {
		println!("th");
	}

}
