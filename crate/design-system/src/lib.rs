#![recursion_limit = "1024"]

pub mod atoms;
pub mod footer;

pub use atoms::*;
pub use footer::Footer;

#[macro_use]
extern crate log;
extern crate web_logger;
