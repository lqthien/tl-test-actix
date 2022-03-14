//! Your Service Description here, etc.

#[macro_use]
extern crate diesel;

use std::io;

#[macro_use]
extern crate log;

pub mod schema;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use std::env;

pub mod accounts;
pub mod assets;
pub mod dashboard;
pub mod packages;
pub mod pages;

use jelly::Server;

pub async fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let _lock = stdout.lock();

    Server::new()
        .register_service(pages::configure)
        .register_service(accounts::configure)
        .register_jobs(accounts::jobs::configure)
        .register_service(dashboard::configure)
        .register_service(assets::configure)
        .register_service(packages::configure)
        .run()
        .await?
        .await
}

pub fn establish_connection() -> PgConnection {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}