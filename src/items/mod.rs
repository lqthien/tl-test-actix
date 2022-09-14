//! URL dispatcher for user account related API endpoints.

use jelly::actix_web::web::{get, post, resource, scope, ServiceConfig};

pub mod forms;
pub mod models;
pub mod views;


pub fn configure(config: &mut ServiceConfig) {
    config.service(
        scope("/items/")
            .service(
                resource("/")
                    .route(get().to(views::list::index))
            )
    );
}
