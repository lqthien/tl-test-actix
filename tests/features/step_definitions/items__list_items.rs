// use cucumber::{given, when, then};
// use thirtyfour::prelude::*;
// use crate::accounts::Account;
// use crate::accounts::forms::NewAccountForm;

// use super::super::world::TestWorld;

// #[given("Given I have logged in")]
// async fn visit_root_page(world: &mut TestWorld) {
//     let form = Form<NewAccountForm> {
//         email: "sonpl@gmail.com",
//         password: "123456",
//     };
//     let db = request.db_pool()?;
//     match Account::register(&form, db).await {
//         Ok(uid) => {
//             request.queue(SendVerifyAccountEmail { to: uid })?;
//         }

//         Err(e) => {
//             error!("Error with registering: {:?}", e);
//             request.queue(SendAccountOddRegisterAttemptEmail {
//                 to: form.email.value.clone(),
//             })?;
//         }
//     }

//     world.driver.get("http://localhost:17002/home").await.unwrap();
// }