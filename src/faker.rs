use super::locale;
use super::locale::Locale;
use std::rand;

pub struct Faker {
    locale: Locale
}

impl Faker {

    pub fn new() -> Faker {
        Faker{locale: locale::en::load()}
    }

    pub fn parse(&self) -> Vec<&'static str> {
        self.locale.address.city_prefix.to_vec()
    }

    pub fn city_prefix(&self) -> String {
        self.sample(self.locale.address.city_prefix.to_vec())
    }

    pub fn sample(&self, arr: Vec<&'static str>) -> String {
        let idx = (rand::random::<uint>() % arr.len());
        arr[idx].to_string()
    }     
   
}