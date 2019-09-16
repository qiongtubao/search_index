use crate::postings::Postings;
use crate::DocId;

struct VecPostings {
    postings: Vec<DocId>,
}
impl VecPostings {
    fn new(postings: Vec<DocId>) -> Self {
        VecPostings { postings }
    }
}
impl Postings for VecPostings {
    type IteratorType = std::vec::IntoIter<DocId>;
    fn iter(&self) -> Self::IteratorType {
        self.postings.clone().into_iter()
    }
}
#[cfg(test)]
mod postings {
    use crate::postings::intersection::IntersectionPostings;
    use crate::postings::vec::VecPostings;
    use crate::DocId;

    #[test]
    fn test_intersection() {
        let left = VecPostings::new(vec![1, 3, 9]);
        let right = VecPostings::new(vec![3, 4, 9, 18]);
        let inter = IntersectionPostings::new(&left, &right);
        let vals: Vec<DocId> = inter.iter().collect();
        assert_eq!(vals, vec!(3, 9));
    }
}
