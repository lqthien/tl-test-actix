use jelly::actix_web::web::{ServiceConfig};

const ASSETS_PATH: &str = "/assets";

pub fn configure(config: &mut ServiceConfig) {
    config.service(actix_files::Files::new(ASSETS_PATH, "./assets").show_files_listing());
}
