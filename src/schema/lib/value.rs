use crate::schema::lib::facet::Facet;

pub type DateTime = chrono::DateTime<chrono::Utc>;
#[derive(Debug)]
pub enum Value {
    /// The str type is used for any text information.
    Str(String),
    /// Unsigned 64-bits Integer `u64`
    U64(u64),
    /// Signed 64-bits Integer `i64`
    I64(i64),
    /// 64-bits Float `f64`
    F64(f64),
    /// Signed 64-bits Date time stamp `date`
    Date(DateTime),
    /// Hierarchical Facet
    Facet(Facet),
    /// Arbitrarily sized byte array
    Bytes(Vec<u8>),
}