enum SpreadsheetCell {
	Int(i32),
	Float(f64),
	Text(String),
}

fn main() {
	
	let v: Vec<i32> = Vec::new();

	let v = vec![1, 2, 3];

	let mut v = Vec::new();

	v.push(5);
	v.push(6);
	v.push(7);
	v.push(8);

	let third: &i32 = &v[2];
	let third: Option<&i32> = v.get(2);

	for i in &mut v {
		*i += 50;
		println!("{}", i);
	}


	let row = vec![
		SpreadsheetCell::Int(3),
		SpreadsheetCell::Float(10.12),
		SpreadsheetCell::Text(String::from("blud")),
		];
	

	for i in &row {
		if let SpreadsheetCell::Int(value) = i {
			println!("{:?}", value);
		}
		if let SpreadsheetCell::Float(value) = i {
			println!("{:?}", value);
		}
		if let SpreadsheetCell::Text(value) = i {
			println!("{:?}", value);
		}
	}

}
