
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure_derive;

extern crate failure;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

mod error;

mod client;
pub mod model;

pub use self::client::Pocketcasts;
