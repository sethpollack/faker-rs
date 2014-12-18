pub use faker::Faker;

include!(concat!(env!("OUT_DIR"), "/locale.rs"))

pub mod faker;
