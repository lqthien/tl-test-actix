//! Packages views.
use jelly::actix_web::{web::Form, HttpRequest, web::Path};
use jelly::prelude::*;
use jelly::Result;

use crate::establish_connection;

use crate::packages::Package;
use crate::packages::forms::NewPackageForm;

/// Returns a list of packages in the system.
pub async fn index(request: HttpRequest) -> Result<HttpResponse> {
    let connection = establish_connection();

    let all_packages = Package::get_all(&connection);

    return request.render(200, "packages/index.html", {
        let mut context = Context::new();
        context.insert("packages", &all_packages);
        context
    })
}

/// Renders a form to submit a new package.
pub async fn new(request: HttpRequest) -> Result<HttpResponse> {
    request.render(200, "packages/new.html", {
        let mut context = Context::new();
        context.insert("form", &NewPackageForm::default());
        context
    })
}

// Creates a new package record.
pub async fn create(
    request: HttpRequest,
    form: Form<NewPackageForm>,
) -> Result<HttpResponse> {
    let mut form = form.into_inner();
    if !form.is_valid() {
        return request.render(400, "packages/new.html", {
            let mut ctx = Context::new();
            ctx.insert("form", &form);
            ctx
        });
    }

    let connection = establish_connection();
    _ = Package::create(&form, &connection);

    request.redirect("/packages/")
}

// Simulates a download and increases download count for specified package
pub async fn download(
    request: HttpRequest,
    Path(id): Path<String>,
) -> Result<HttpResponse> {
    let connection = establish_connection();
    let uid = id.parse::<i32>().unwrap();

    let mut package = Package::find(uid, &connection);
    package.downloads_count += 1;

    package.update(&connection);

    request.redirect("/packages/")
}
