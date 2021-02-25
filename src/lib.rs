use std::env;
use std::fmt;

mod conf;
mod environment;
mod req;
mod routes;

pub use conf::WrapperConfig;
pub use req::{crypto_test, ForwordRequest};
pub use routes::config;

pub const BIND_ADDRESS: &'static str = "127.0.0.1:8080";
