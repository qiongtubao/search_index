use crate::codec::{Codec, SegmentOutput};
use crate::serial::TermCursor;
use crate::serial::{DocCursor, SerializableSegment};
use byteorder::LittleEndian;
use byteorder::WriteBytesExt;
use core::fmt::Alignment::Left;
use fst::MapBuilder;
use std::io::Write;
pub struct SimpleCodec;
impl SimpleCodec {
    fn write_postings<D: DocCursor, W: Write>(
        mut doc_it: D,
        postings: &mut W,
    ) -> crate::error::Result<usize> {
        let mut written_bytes: usize = 4;
        postings.write_u32::<LittleEndian>(doc_it.len() as u32);
        for doc_id in doc_it {
            postings.write_u32::<LittleEndian>(doc_id as u32);
            written_bytes += 4;
        }
        Ok(written_bytes)
    }
}

impl Codec for SimpleCodec {
    fn write<'a, I: SerializableSegment<'a>, W: Write>(
        index: &'a I,
        output: &'a SegmentOutput<'a, W>,
    ) -> crate::error::Result<usize> {
        let term_trie_builder_result = MapBuilder::new(output.terms());
        if term_trie_builder_result.is_err() {
            return Err(crate::error::Error::IOError(String::from(
                "Failed creating the term builder",
            )));
        }
        let mut term_buffer: String = String::new();
        let mut term_trie_builder = term_trie_builder_result.unwrap();
        let mut term_cursor = index.term_cursor();
        let mut offset: usize = 0;
        let mut postings_output = output.postings();
        loop {
            match term_cursor.next() {
                Some((term, doc_it)) => {
                    term.write_into(&mut term_buffer);
                    match term_trie_builder.insert(&term_buffer, offset as u64) {
                        Ok(_) => {}
                        Err(_) => {
                            return Err(crate::error::Error::IOError(String::from(
                                "Failed while inserting into the fst",
                            )))
                        }
                    }
                    offset += SimpleCodec::write_postings(doc_it, &mut postings_output)?;
                }
                None => {
                    break;
                }
            }
        }
        Ok(0)
    }
}
