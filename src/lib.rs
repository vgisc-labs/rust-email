#![cfg_attr(all(feature = "nightly", test), feature(test))]

extern crate base64;
extern crate chrono;
extern crate encoding;
extern crate rand;

#[macro_use]
extern crate lazy_static;

pub use crate::address::{Address, Mailbox};
pub use crate::header::{FromHeader, Header, HeaderIter, HeaderMap, ToFoldedHeader, ToHeader};
pub use crate::message::{MimeMessage, MimeMultipartType};

mod address;
mod header;
mod message;
pub mod mimeheaders;
pub mod results;
pub mod rfc2045;
pub mod rfc2047;
pub mod rfc5322;
pub mod rfc822;
