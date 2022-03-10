//! Packages resource.

use jelly::actix_web::web::{resource, scope, ServiceConfig};

pub mod views;
pub mod models;

pub use models::Package;

pub fn configure(config: &mut ServiceConfig) {
    config.service(
        scope("/packages/")
            // Index
            .service(resource("").to(views::packages_index))
    );
}
