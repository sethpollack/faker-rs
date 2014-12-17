extern crate faker;
use faker::Faker;


fn main() {
	let faker = Faker::new("en");
	println!("{}", faker.parse());
}