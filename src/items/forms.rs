use jelly::forms::{EmailField, PasswordField, TextField, Validation};
use serde::{Deserialize, Serialize};

fn default_redirect_path() -> String {
    "/".into()
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct ItemForm {
    pub name: TextField,

    #[serde(default = "default_redirect_path")]
    pub redirect: String,
}


#[derive(Debug, Default, Deserialize, Serialize)]
pub struct NewItemForm {
    pub new_item_name: TextField,
}

impl Validation for NewItemForm {
    fn is_valid(&mut self) -> bool {
        self.new_item_name.len() > 0
    }
}
