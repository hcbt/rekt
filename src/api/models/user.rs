use crate::api_error::ApiError;
use crate::db;
use crate::schema::users;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, AsChangeset)]
#[table_name = "users"]
pub struct UserMessage {
    pub email: String,
    pub name: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

impl User {
    pub fn find_all() -> Result<Vec<Self>, ApiError> {
        let conn = &mut db::connection()?;

        let users = users::table
            .load::<User>(conn)?;

        Ok(users)
    }

    pub fn find(id: Uuid) -> Result<Self, ApiError> {
        let conn = &mut db::connection()?;

        let user = users::table
            .filter(users::id.eq(id))
            .first(conn)?;

        Ok(user)
    }

    pub fn create(user: UserMessage) -> Result<Self, ApiError> {
        let conn = &mut db::connection()?;

        let user = User::from(user);
        let user = diesel::insert_into(users::table)
            .values(user)
            .get_result(conn)?;

        Ok(user)
    }

    pub fn update(id: Uuid, user: UserMessage) -> Result<Self, ApiError> {
        let conn = &mut db::connection()?;

        let user = diesel::update(users::table)
            .filter(users::id.eq(id))
            .set(user)
            .get_result(conn)?;

        Ok(user)
    }

    pub fn delete(id: Uuid) -> Result<usize, ApiError> {
        let conn = &mut db::connection()?;

        let res = diesel::delete(
                users::table
                    .filter(users::id.eq(id))
            )
            .execute(conn)?;

        Ok(res)
    }
}

impl From<UserMessage> for User {
    fn from(users: UserMessage) -> Self {
        User {
            id: Uuid::new_v4(),
            email: users.email,
            name: users.name,
            password: users.password,
            created_at: Utc::now().naive_utc(),
            updated_at: None,
        }
    }
}