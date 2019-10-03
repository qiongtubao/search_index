use uuid::Uuid;
use std::str::FromStr;
use crate::core::segment::error::SegmentIdParseError;
use std::cmp::Ordering;
use std::fmt;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SegmentId(Uuid);


