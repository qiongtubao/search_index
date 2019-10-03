use crate::schema::lib::field::options::text::TextOptions;
use crate::schema::lib::field::options::int::IntOptions;
//FieldEntry 需要debug
#[derive(Debug)]
pub enum FieldType {
    Str(TextOptions),
    /// Unsigned 64-bits integers field type configuration
    U64(IntOptions),
    /// Signed 64-bits integers 64 field type configuration
    I64(IntOptions),
    /// 64-bits float 64 field type configuration
    F64(IntOptions),
    /// Signed 64-bits Date 64 field type configuration,
    Date(IntOptions),
    /// Hierachical Facet
    HierarchicalFacet,
    /// Bytes (one per document)
    Bytes,
}
