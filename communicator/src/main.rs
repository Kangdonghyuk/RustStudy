extern crate communicator;

fn main() {
	communicator::client::connect();

	communicator::tests::it_works();
}

