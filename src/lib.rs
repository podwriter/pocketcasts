#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate failure_derive;

mod error;

mod client;
pub mod model;

pub use self::client::Pocketcasts;
