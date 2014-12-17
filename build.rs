use std::os;
use std::io::File;
use std::io::fs;
use std::str;

fn main() {
    let dst = Path::new(os::getenv("OUT_DIR").unwrap());
    let mut f = File::create(&dst.join("locale.rs")).unwrap();
    let path = &Path::new("./locales");
		let files = fs::readdir(path).unwrap();

    f.write_line(
    	"pub mod locale {\nuse std::collections::HashMap;\n    pub fn load() -> HashMap<&'static str, &'static str> {\n        let mut locales = HashMap::new();"
    ).unwrap();
		for path in files.iter() {
			let filename = str::replace(path.filename_str().unwrap(), ".toml", "");
			let contents = File::open(path).read_to_string().unwrap();
			f.write_line(
	    	format!("        locales.insert(\"{}\",{}{}{});", filename, "r#############################\"", contents, "\"#############################").as_slice()
	    ).unwrap();
		}
    f.write_line(
    	"        locales \n    }    \n}"
    ).unwrap();
}