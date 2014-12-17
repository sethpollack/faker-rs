extern crate toml;
use super::locale;
use std::rand;

pub struct Faker {
    locale: toml::Value
}

impl Faker {

    pub fn new(locale: &str) -> Faker {
        let map = locale::load();
        let value: toml::Value = from_str(map.get(locale).unwrap().as_slice()).unwrap();
        Faker{locale: value}
    }

    pub fn parse(&self) -> String {
        self.locale.lookup("address.city_prefix").unwrap().to_string()
    }

    pub fn sample(&self, arr: Vec<&'static str>) -> String {
        let idx = (rand::random::<uint>() % arr.len()) + 1u;
        arr[idx].to_string()
    }     
   
}

