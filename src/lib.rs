pub use faker::Faker;

include!(concat!(env!("OUT_DIR"), "/locale.rs"))

mod faker;
