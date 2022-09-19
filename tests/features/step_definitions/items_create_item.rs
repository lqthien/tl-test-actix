use cucumber::{given, when, then};
use jelly::forms::{TextField, EmailField, PasswordField};
use mainlib::{accounts::{forms::NewAccountForm, Account}, test::DB_POOL};
use thirtyfour::prelude::*;

use super::super::world::TestWorld;

#[given("Given I have logged in")]
async fn visit_root_page(world: &mut TestWorld) {
    let new_account_form = NewAccountForm {
        name: TextField { value: "drwpls".to_string(), errors: vec![] },
        email: EmailField { value: "pls@pls.pls".to_string(), errors: vec![] },
        password: PasswordField {value: "drwpls".to_string(),errors:vec![], hints: vec![] },
    };
    
    let uid = Account::register(&new_account_form, &DB_POOL).await.unwrap();
    
    Account::mark_verified(uid, &DB_POOL).await.unwrap();

    let mut caps = DesiredCapabilities::chrome();
    caps.set_headless();

    world.go_to_root_url().await;
    world.driver.get("http://localhost:17002/home").await.unwrap();

    let email_field = world.driver.find_element(By::Name("email")).await.unwrap();
    email_field.send_keys("pls@pls.pls").await.unwrap();
    let password_field = world.driver.find_element(By::Name("password")).await.unwrap();
    password_field.send_keys("drwpls").await.unwrap();
    let login_button = world.driver.find_element(By::ClassName("login_btn")).await.unwrap();
    login_button.click().await.unwrap();
}

#[when("When I click on the Items button")]
async fn click_on_items_button(world: &mut TestWorld) {
    world.driver.find_element(By::LinkText("Items")).await.unwrap().click().await.unwrap();
}

#[then("Then I should have see my items and A button named Create new Item")]
async fn see_my_items_and_create_new_item_button(world: &mut TestWorld) {
    let heading = world.driver.find_element(By::Tag("h1")).await.unwrap();
    let heading_text = heading.text().await.unwrap();
    assert_eq!(heading_text, "Items:");

    let items = world.driver.find_element(By::Tag("p")).await.unwrap();
    let items_text_text = items.text().await.unwrap();
    assert_eq!(items_text_text, "");

    let create_new_item_button = world.driver.find_element(By::ClassName("create_item_btn")).await.unwrap();
    let create_new_item_button_text = create_new_item_button.text().await.unwrap();
    assert_eq!(create_new_item_button_text, "Create new Item");

}

#[when("When I fill in text field with item name and click to Create new Item")]
async fn fill_in_text_field_with_item_name_and_click_to_create_new_item(world: &mut TestWorld) {
    let item_name_field = world.driver.find_element(By::Id("new_name_text_field")).await.unwrap();
    item_name_field.send_keys("test item").await.unwrap();
    let create_new_item_button = world.driver.find_element(By::ClassName("create_item_btn")).await.unwrap();
    create_new_item_button.click().await.unwrap();
}

#[then("Then I should see my item in the list")]
async fn see_my_item_in_the_list(world: &mut TestWorld) {
    let items = world.driver.find_element(By::Tag("p")).await.unwrap();
    let items_text_text = items.text().await.unwrap();
    assert_eq!(items_text_text, "test item");
}