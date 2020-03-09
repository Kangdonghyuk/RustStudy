mod most {
	pub fn middle_function() {
		println!("middle");
	}
	fn middle_secret_function() {}
	pub	mod inside {
		pub fn inner_function() {
		}
		fn secret_function() {}
	}
}


pub mod tests {
    pub fn it_works() {
    
		println!("hello");
	
	}
}

pub fn try_me() {
	most::middle_function();
	most::inside::inner_function();

}
