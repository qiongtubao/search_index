use crate::serial::SerializableSegment;
use std::io::Write;
mod debug;
mod simple;
pub trait Codec {
    fn write<'a, I: SerializableSegment<'a>, W: Write>(
        index: &'a I,
        output: &'a SegmentOutput<'a, W>,
    ) -> crate::error::Result<usize>;
}
pub trait SegmentOutput<'a, W: Write> {
    fn terms(&self) -> W;
    fn postings(&self) -> W;
}
