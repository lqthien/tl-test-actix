use jelly::forms::{TextField, Validation};
use serde::{Deserialize, Serialize};

use crate::schema::packages;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct NewPackageForm {
    pub name: TextField,
    pub description: TextField
}

impl Validation for NewPackageForm {
    fn is_valid(&mut self) -> bool {
        self.name.is_valid()
        && self.description.is_valid()
    }
}

#[derive(Insertable)]
#[table_name="packages"]
pub struct NewPackage<'a> {
    pub name: &'a str,
    pub description: &'a str
}