use jelly::actix_web::{web::Form, HttpRequest};
use jelly::prelude::*;
use jelly::request::{Authentication, DatabasePool};
use jelly::Result;

use crate::accounts::forms::LoginForm;
use crate::accounts::Account;
use crate::items::models::*;

/// The login form.
pub async fn form(request: HttpRequest) -> Result<HttpResponse> {
    if request.is_authenticated()? {
        return request.redirect("/dashboard/");
    }

    request.render(200, "accounts/login.html", {
        let mut ctx = Context::new();
        ctx.insert("form", &LoginForm::default());
        ctx
    })
}

pub async fn index(request: HttpRequest) -> Result<HttpResponse> {
    if !request.is_authenticated()? {
        return request.redirect("/home");
    }

    return request.render(200, "items/index.html", {
        let mut ctx = Context::new();
        ctx
    });

}

