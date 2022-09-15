// Implements a basic Account model, with support for creating/updating/deleting
// users, along with welcome email and verification.

use diesel::{Queryable, Identifiable, AsChangeset, Insertable};
use diesel::prelude::*;

use jelly::accounts::{OneTimeUseTokenGenerator, User};
use jelly::chrono::{DateTime, Utc, offset};
use jelly::djangohashers::{check_password, make_password};
use jelly::error::Error;
use jelly::serde::{Deserialize, Serialize};
use jelly::DieselPgPool;

use crate::schema::items;
use crate::schema::items::dsl::*;


use super::forms::{LoginForm, NewAccountForm};

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, AsChangeset)]
pub struct Item {
    pub id: i32,
    pub uid: i32,
    pub name: String,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

impl Item {
    pub async fn get_list_by_uid(uid_: i32, pool: &DieselPgPool) -> Result<Vec<Self>, Error> {
        return Ok(vec![]);
    }

    pub async fn get_by_id(id_: i32, pool: &DieselPgPool) -> Result<Self, Error> {
        let connection = pool.get()?;
        let result = items
            .find(id_)
            .first::<Item>(&connection)?;
        return Ok(result);
    }

    pub async fn create(dbpool: &DieselPgPool, uid_: i32, name_: String) -> Result<Self, Error> {
        let conn = dbpool.get().unwrap();

        let newItem = NewItem::from_uid_name(uid_, name_);

        let item = diesel::insert_into(items::table)
            .values(&newItem)
            .get_result(&conn)?;
        return Ok(item);
    } 
}


#[cfg(test)]
mod tests {
    use crate::accounts::{Account};
    use crate::accounts::forms::NewAccountForm;
    use crate::items::models::Item;
    use crate::test::{DB_POOL, DatabaseTestContext};
    use jelly::forms::{EmailField, PasswordField, TextField};

    pub async fn setup_user(email: Option<String>, password: Option<String>) -> i32 {
        let form = NewAccountForm {
            email: EmailField {
                value: email.unwrap_or_else(|| "email@host.com".to_string()),
                errors: vec![],
            },
            password: PasswordField {
                value: password.unwrap_or_else(|| "So$trongpas0word!".to_string()),
                errors: vec![],
                hints: vec![],
            },
            name: TextField {
                value: "Test name".to_string(),
                errors: vec![],
            },
        };

        let uid = Account::register(&form, &DB_POOL).await.unwrap();
        let _ = Account::mark_verified(uid, &DB_POOL).await;
        uid
    }

    #[actix_rt::test]
    async fn test_items_get_by_id() {
        crate::test::init();
        let _ctx = DatabaseTestContext::new();

        let uid = setup_user(None, None).await;

        

        let item = Item::create(&DB_POOL, uid, "Test item".to_string()).await.unwrap();
        let get_item = Item::get_by_id(item.id, &DB_POOL).await.unwrap();
        assert_eq!(item.uid, uid);
    }

    #[actix_rt::test]
    async fn test_items_create() {
        crate::test::init();
        let _ctx = DatabaseTestContext::new();
        let uid = setup_user(None, None).await;
        let item = Item::create(&DB_POOL, uid, "Test item".to_string()).await.unwrap();

        let get_item = Item::get_by_id(item.id, &DB_POOL).await.unwrap();
        assert_eq!(item.name, "Test item");
        assert_eq!(item.name, get_item.name);
    }

    // Sample unit test
    #[actix_rt::test]
    async fn test_items_list() {

        crate::test::init();
        let ctx = DatabaseTestContext::new();
        let uid = setup_user(None, None).await;
        
        let result = super::Item::get_list_by_uid(uid, &DB_POOL).await;
        
        assert!(result.is_ok());

        assert!(result.unwrap().len() == 0);

        // crate::test::init();
        // let _ctx = DatabaseTestContext::new();
        // let form = NewAccountForm {
        //     name: TextField { value: "Test name".to_string(), errors: vec![] },
        //     email: EmailField { value: "email@host.com".to_string(), errors: vec![] },
        //     password: PasswordField { value: "xxyyzz".to_string(), errors: vec![], hints: vec![] },
        // };
        // let uid = Account::register(&form, &DB_POOL).await.unwrap();
        // let account = Account::get(uid, &DB_POOL).await.unwrap();
        // assert_eq!(account.email, "email@host.com");
    }
}

#[derive(Insertable)]
#[table_name="items"]
pub struct NewItem {
    pub uid: i32,
    pub name: String
}

impl NewItem {
    fn from_uid_name(uid_: i32, name_: String) -> Self {
        return NewItem {
            uid: uid_,
            name: name_,
        };
    }
}