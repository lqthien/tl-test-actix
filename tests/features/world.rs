use std::convert::Infallible;

use async_trait::async_trait;
use cucumber::{World, WorldInit};
use thirtyfour::prelude::*;

// `World` is your shared, likely mutable state.
#[derive(Debug, WorldInit)]
pub struct TestWorld {
    pub driver: WebDriver,
    pub root_url: String
}

// `World` needs to be implemented, so Cucumber knows how to construct it
// for each scenario.
#[async_trait(?Send)]
impl World for TestWorld {
    // We do require some error type.
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self {
            driver: {
                let mut caps = DesiredCapabilities::chrome();
                caps.add_chrome_arg("--no-sandbox").unwrap();
                caps.add_chrome_arg("--headless").unwrap();
                WebDriver::new("http://localhost:4444", &caps).await.unwrap()
            },
            root_url: "http://localhost:17002".to_string()
        })
    }
}

impl TestWorld {
    pub async fn go_to_root_url(&self) {
        self.driver.get(&self.root_url).await.unwrap()
    }

    pub async fn close_browser(&self) {
        self.driver.handle.cmd(thirtyfour::common::command::Command::DeleteSession).await.unwrap();
    }
}
