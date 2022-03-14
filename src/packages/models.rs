use jelly::chrono::{DateTime, Utc};
use jelly::error::Error;
use jelly::serde::{Deserialize, Serialize};
use jelly::sqlx::{self, postgres::PgPool};

use diesel;
use diesel::prelude::*;

use crate::schema::packages::dsl::*;
use crate::schema::packages;
use crate::packages::forms::*;

/// A Package Object.
#[derive(Queryable, Serialize, Deserialize, Identifiable, AsChangeset)]
pub struct Package {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub downloads_count: i32
}

impl Package {

    pub fn get_all(connection: &PgConnection) -> Vec<Self> {
        let results = packages
        .load::<Package>(connection)
        .expect("Error loading packages");
        
        results
    }

    pub fn create<'a>(form: &NewPackageForm, connection: &PgConnection) -> Package {
        let new_package = NewPackage {
            name: &form.name.value,
            description: &form.description.value
        };
    
        diesel::insert_into(packages::table)
            .values(&new_package)
            .get_result(connection)
            .expect("Error saving new package")
    }

    pub fn update(self: &Self, connection: &PgConnection) -> Package {
        self.save_changes(connection).unwrap()
    }

    pub fn find(uid: i32, connection: &PgConnection) -> Package {
        return packages::table.find(uid).first::<Package>(connection).expect("Error loading package");
    }

    // pub async fn create_sample(pool: &PgPool) -> Result<i32, Error> {
    //     Ok(sqlx::query!(
    //         "
    //         INSERT INTO packages (name, description)
    //         VALUES ($1, $2)
    //         RETURNING id
    //     ",
    //         "Sample Package 1",
    //         "Sample Description 1"
    //     )
    //     .fetch_one(pool)
    //     .await?
    //     .id)
    // }
}
