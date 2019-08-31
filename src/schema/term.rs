use crate::schema::field::Field;

pub struct Term<'a> {
    pub field: &'a Field,
    pub text: &'a str,
}