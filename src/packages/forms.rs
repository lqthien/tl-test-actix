use jelly::forms::{TextField, Validation};
use serde::{Deserialize, Serialize};

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
