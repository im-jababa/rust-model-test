use crate::feature::user::*;

#[derive(Debug)]
pub struct User {
    pk: PK,
    id: ID,
    provider: provider::Kind,
}
