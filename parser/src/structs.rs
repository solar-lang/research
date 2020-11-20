// Holds all information about parsing record types in solar

use crate::{generics::*, identifier::Identifier};

pub struct Structure<'a> {
    pub public: bool,
    pub name: Identifier<'a>,
    pub generics: GenericHeader<'a>,
}
