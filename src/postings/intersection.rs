use crate::index::core::postings::Postings;
use crate::index::core::DocId;
use std::fmt::{self, Debug, Formatter};
pub struct IntersectionPostings<'a, LeftPostingsType, RightPostingsType>
where
    LeftPostingsType: Postings + 'static,
    RightPostingsType: Postings + 'static,
{
    left: &'a LeftPostingsType,
    right: &'a RightPostingsType,
}

pub struct IntersectionIterator<LeftPostingsType: Postings, RightPostingsType: Postings> {
    left: LeftPostingsType::IteratorType,
    right: RightPostingsType::IteratorType,
    next_left: Option<DocId>,
    next_right: Option<DocId>,
}

impl<LeftPostingsType: Postings, RightPostingsType: Postings> Iterator
    for IntersectionIterator<LeftPostingsType, RightPostingsType>
{
    type Item = DocId;
    fn next(&mut self) -> Option<DocId> {
        loop {
            match (self.next_left, self.next_right) {
                (_, None) => {
                    return None;
                }
                (None, _) => {
                    return None;
                }
                (Some(left_val), Some(right_val)) => {
                    if left_val < right_val {
                        self.next_left = self.left.next();
                    } else if left_val > right_val {
                        self.next_right = self.right.next();
                    } else {
                        self.next_left = self.left.next();
                        self.next_right = self.right.next();
                        return Some(left_val);
                    }
                }
            }
        }
    }
}

impl<'a, LeftPostingsType, RightPostingsType> Postings
    for IntersectionPostings<'a, LeftPostingsType, RightPostingsType>
where
    LeftPostingsType: Postings + 'static,
    RightPostingsType: Postings + 'static,
{
    type IteratorType = IntersectionIterator<LeftPostingsType, RightPostingsType>;
    fn iter(&self) -> Self::IteratorType {
        let mut left = self.left.iter();
        let mut right = self.right.iter();
        let next_left = left.next();
        let next_right = right.next();
        IntersectionIterator {
            left,
            right,
            next_left,
            next_right,
        }
    }
}
impl<'a, L: Postings + 'static, R: Postings + 'static> Debug for IntersectionPostings<'a, L, R> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        let postings_lists: Vec<DocId> = self.iter().collect();
        write!(f, "Postings({:?})", postings_lists);
        Ok(())
    }
}

pub fn intersection<'a, LeftPostingsType, RightPostingsType>(
    left: &'a LeftPostingsType,
    right: &'a RightPostingsType,
) -> IntersectionPostings<'a, LeftPostingsType, RightPostingsType>
where
    LeftPostingsType: Postings + 'static,
    RightPostingsType: Postings + 'static,
{
    IntersectionPostings {
        left: left,
        right: right,
    }
}
