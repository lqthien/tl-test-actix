//! Packages views.
use jelly::actix_web::{web::Form, HttpRequest, web::Path};
use jelly::prelude::*;
use jelly::Result;

use crate::packages::Package;
use crate::packages::forms::NewPackageForm;

/// Returns a list of packages in the system.
pub async fn index(request: HttpRequest) -> Result<HttpResponse> {
    let db = request.db_pool()?;
    let packages: Vec<Package> = match Package::get_all(db).await {
        Ok(packages) => {
            packages
        }
        Err(_) => {
            Package::create_sample(db).await?;
            Package::get_all(db).await?
        }
    };

    return request.render(200, "packages/index.html", {
        let mut context = Context::new();
        context.insert("packages", &packages);
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

    let db = request.db_pool()?;
    match Package::create(&form, db).await {
        Ok(_id) => {
            request.redirect("/packages/")
        }

        Err(e) => {
            error!("Error with creating package: {:?}", e);
            request.render(400, "packages/new.html", {
                let mut ctx = Context::new();
                ctx.insert("form", &form);
                ctx
            })
        }
    }
}

// Simulates a download and increases download count for specified package
pub async fn download(
    request: HttpRequest,
    Path(id): Path<String>,
) -> Result<HttpResponse> {
    let db = request.db_pool()?;
    let uid = id.parse::<i32>().unwrap();
    let mut package = Package::get(uid, db).await?;
    package.downloads_count += 1;

    match package.update(db).await {
        Ok(_id) => {
            request.redirect("/packages/")
        }

        Err(e) => {
            error!("Error with updating package: {:?}", e);
            request.redirect("/packages/")
        }
    }
}
