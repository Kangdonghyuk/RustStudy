enum IpAddrKind {
	V4,
	V6,
}

fn main() {

	let four = IpAddrKind::V4;
	let six = IpAddrKind::V6;

	let temp: i32;
	
	route(IpAddrKind::V4);
	route(IpAddrKind::V6);

}

fn route(ip_type: IpAddrKind) {}