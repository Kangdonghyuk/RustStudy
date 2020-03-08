struct User {
	username: String,
	email: String,
	sign_in_count: u64,
	active: bool,
}

fn main() {


	let user1 = User {
		email: String::from("someone@example.com"),
		username: String::from("someusername123"),
		active: true,
		sign_in_count: 1,
	};

	let user2 = User {
		email: String::from("another@example.com"),
		username: String::from("anotherusername567"),
		..user1
	};

	println!("{}, {}, {}, {}", user1.email, user1.username, user1.active,
			user1.sign_in_count);

	println!("{}, {}, {}", user2.email, user2.username, user2.active);


	let email: String = String::from("test");
	let name: String = String::from("test");
	let user3 = build_user(email, name);

	println!("{}, {}", user3.email, user3.username);

}

fn build_user(email: String, username: String) -> User {
	User {
		email,
		username,
		active: true,
		sign_in_count: 1,
	}
}
