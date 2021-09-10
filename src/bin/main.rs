use lib::NFT;

fn main() {
	let mut some = NFT::new(
		vec!["one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), "six".to_string(), "seven".to_string(), "eight".to_string()]
		);
	println!("{:?}", some.get_next_bundle(4)); 
	println!("{:?}", some.get_next_bundle(4));
}