use crate::schema::field::Field;
#[derive(Debug)]
pub struct Term<'a> {
    pub field: Field,
    pub text: &'a str,
}