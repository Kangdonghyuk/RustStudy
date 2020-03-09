fn main() {
	
	let mut s = String::new();

	let data = "initial contents";

	let s = data.to_string();

	let s = "initial contents".to_string();

	let mut s = String::from("foo");
	s.push_str("bar");

	println!("{}", s);

	let mut s1 = String::from("foo");
	let s2 = "bar";
	s1.push_str(&s2);
	println!("sw is {}", s2);

	let s1 = String::from("tic");
	let s2 = String::from("tac");
	let s3 = String::from("toe");

	let s = format!("{}-{}-{}", s1, s2, s3);

	println!("{}", s);

	for c in s.chars() {
		println!("{}", c);
	}

	for c in s.bytes() {
		println!("{}", c);
	}

}
