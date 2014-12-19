extern crate faker;
use faker::Faker;


fn main() {
	let faker = Faker::new("en");
	println!("{}", faker.city_prefix());
}



// extern crate toml;
// use std::os;
// use std::io::File;
// use std::io::fs;
// use std::str;


// fn main() {
// 	let p = &Path::new("./locales");
// 	let files = fs::readdir(p).unwrap();
// 	// let toml = File::open(p).read_to_string().unwrap();

// 	let toml = files.iter().map(|p|
// 		File::open(p).read_to_string().unwrap()
// 	).collect::<Vec<String>>().connect("");

// 	let value: toml::Value = from_str(toml.as_slice()).unwrap();
// 	let langs = value.lookup("lang.en.address.city_prefix").unwrap().as_slice().unwrap().to_vec();

// 	for lang in langs.iter(){
// 		println!("{}",  lang);
// 	}
	
// }


