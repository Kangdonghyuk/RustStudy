enum IpAddrKind {
	V4,
	V6,
}

fn main() {

	let four = IpAddrKind::V4;
	let six = IpAddrKind::V6;

	let temp: Option<i32> = Some(5);
	let tt: i32 = 50;

	let temp: i32 = 100;

	route(IpAddrKind::V4);
	route(IpAddrKind::V6);

	println!("{}", tt + temp);

}

fn route(ip_type: IpAddrKind) {}
