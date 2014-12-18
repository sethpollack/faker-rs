extern crate toml;
use std::os;
use std::io::File;
use std::io::fs;
use std::str;

fn main(){
    let dst = Path::new(os::getenv("OUT_DIR").unwrap());
    let mut f = File::create(&dst.join("locale.rs")).unwrap();
    let p = &Path::new("./locales");
    let files = fs::readdir(p).unwrap();
    f.write_line("

        pub mod locale {

            pub struct Address {
                pub city_prefix: Vec<&'static str>
            }
            pub struct Locale {
                pub address: Address
            }            

    ").unwrap();
    for path in files.iter() {
        let filename = rename_mod(str::replace(path.filename_str().unwrap(), ".toml", ""));
        let contents = File::open(path).read_to_string().unwrap();
        let value: toml::Value = from_str(contents.as_slice()).unwrap();
        f.write_line(format!(" pub mod {}", filename).as_slice()).unwrap();
        f.write_line("
                    {
                use super::Locale;
                use super::Address;
                pub fn load() -> Locale{
                    Locale {
                        address: Address{
        ").unwrap();
        f.write_line(format!("city_prefix: vec! {},", value.lookup("address.city_prefix").unwrap().to_string()).as_slice());
        f.write_line("
                    }
                }
            }
        }
        ").unwrap();    
    }
    f.write_line("
        }
    ").unwrap();


}

fn rename_mod(string: String) -> String {
    let str = string.chars().map(|c| c.to_lowercase().to_string()).collect::<Vec<String>>().connect("");
    str::replace(str.as_slice(), "-", "_")
}



