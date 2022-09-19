use jelly::actix_web::{web::Form, HttpRequest};
use jelly::prelude::*;
use jelly::request::{Authentication, DatabasePool};
use jelly::Result;

use crate::items::forms::NewItemForm;
use crate::items::models::*;

pub async fn create_item(request: HttpRequest, form: Form<NewItemForm>) -> Result<HttpResponse> {
    if !request.is_authenticated()? {
        return request.redirect("/home/");
    }

    let mut form = form.into_inner();
    if !form.is_valid() {
        return request.render(400, "item", {
            let mut ctx = Context::new();
            ctx.insert("form", &form);
            ctx
        });
    }

    let user = request.user()?;
    let uid = user.id;
    let new_item_name = form.new_item_name.value.clone();



    let db = request.db_pool()?;

    match Item::create(db, uid, new_item_name).await {
        Ok(uid) => {
            request.redirect("/items/");
        }

        Err(e) => {
            error!("Error with registering: {:?}", e);
            request.redirect("/items/");
        }
    }


    // No matter what, just appear as if it worked.
    request.redirect("/items/")
}


