//! Packages resource.

use jelly::actix_web::web::{post, resource, scope, ServiceConfig};

pub mod views;
pub mod models;
pub mod forms;

pub use models::Package;

pub fn configure(config: &mut ServiceConfig) {
    config.service(
        scope("/packages/")
            // Index
            .service(resource("").to(views::index))
            // New
            .service(resource("/new").to(views::new))
            // Create
            .service(resource("/create").route(post().to(views::create)))
            // Update Downloads Count
            .service(resource("/{id}/download").to(views::download))
    );
}
