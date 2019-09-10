use crate::directory::Directory;
use crate::serial::FieldCursor;
use crate::schema::field::Field;


pub struct SegmentIndexReader {
    directory: Directory,
}