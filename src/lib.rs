#[macro_use]
extern crate lazy_static;

use std::io::Write;

mod analyzer;
mod directory;
mod schema;
mod writer;
mod reader;
pub type DocId = usize;


pub trait Flushable {
    fn flush<W: Write>(&self, writer: &mut W) -> Result<usize, std::io::Error>;
}