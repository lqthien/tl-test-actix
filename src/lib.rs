//! Your Service Description here, etc.

use std::io;

#[macro_use]
extern crate log;

pub mod accounts;
pub mod dashboard;
pub mod pages;
pub mod assets;
pub mod packages;

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
