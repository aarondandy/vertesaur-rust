#[link(
	name = "vertesaur",
	vers = "0.0.1",
	author = "Aaron Dandy",
	url = "https://github.com/aarondandy/vertesaur-rust"
	)]
#[desc = "A computational geometry and math library for Rust with potential medical, gaming, and GIS uses."]
#[license = "MIT"]
#[crate_type = "lib"]

mod vector;

pub mod vertesaur {
	pub use vector::SimpleStruct;
}

fn main() { }