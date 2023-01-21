pub mod client;
pub mod error;
pub mod model;
pub mod options;
pub mod request;

#[cfg(feature = "blocking")]
pub mod blocking;

#[cfg(test)]
mod tests;

//https://opentdb.com/api_config.php

pub mod prelude {
    pub use crate::{
        options::*,
        request::{OwnedRequest, Request},
        client::*,
        error::*
    };
}