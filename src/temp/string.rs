use crate::vec::{Vec};
use core::str::Utf8Error;


pub struct FromUtf8Error {
    bytes: Vec<u8>,
    error: Utf8Error,
}


pub struct String {
    vec: crate::vec::Vec<u8>,
}

impl String {
    pub fn new() -> String {
        String { vec: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> String {
        String { vec: Vec::with_capacity(capacity) }
    }
}
