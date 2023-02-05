use crate::api_error::ApiError;
use crate::db;
use crate::schema::users;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use argon2::Config;
use rand::Rng;
use uuid::Uuid;

#[derive(Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = users)]
pub struct UserMessage {
    pub email: String,
    #[serde(default)]
    pub name: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

impl User {
    pub fn get_all() -> Result<Vec<Self>, ApiError> {
        let conn = &mut db::connection()?;

        let users = users::table
            .load::<User>(conn)?;

        Ok(users)
    }

    pub fn get(id: Uuid) -> Result<Self, ApiError> {
        let conn = &mut db::connection()?;

        let user = users::table
            .filter(users::id.eq(id))
            .first(conn)?;

        Ok(user)
    }

    pub fn get_by_email(email: String) -> Result<Self, ApiError> {
        let conn = &mut db::connection()?;

        let user = users::table
            .filter(users::email.eq(email))
            .first(conn)?;

        Ok(user)
    }
 
    pub fn create(user: UserMessage) -> Result<Self, ApiError> {
        let conn = &mut db::connection()?;

        let mut user = User::from(user);
        user.hash_password()?;
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

    pub fn hash_password(&mut self) -> Result<(), ApiError> {
        let salt: [u8; 32] = rand::thread_rng().gen();
        let config = Config::default();

        self.password = argon2::hash_encoded(self.password.as_bytes(), &salt, &config)
            .map_err(|e| ApiError::new(500, format!("Failed to hash password: {}", e)))?;

        Ok(())
    }

    pub fn verify_password(&self, password: &[u8]) -> Result<bool, ApiError> {
        argon2::verify_encoded(&self.password, password)
            .map_err(|e| ApiError::new(500, format!("Failed to verify password: {}", e)))
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