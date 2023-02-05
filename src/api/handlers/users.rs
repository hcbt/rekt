use crate::api_error::ApiError;
use crate::api::models::users::{User, UserMessage};
use actix_web::{delete, get, put, web, HttpResponse};
use actix_identity::Identity;
use serde_json::json;
use uuid::Uuid;

// Get all users.
#[get("/users")]
async fn find_all(user_identity: Option<Identity>) -> Result<HttpResponse, ApiError> {
    if let Some(_user) = user_identity {
        // The user is authenticated
        let users = User::get_all()?;
        Ok(HttpResponse::Ok().json(users))
    } else {
        // The user is not authenticated
        Err(ApiError::new(401, "Unauthorized".to_owned()))
    }
}

// Get user by ID.
#[get("/users/{id}")]
async fn find(user_identity: Option<Identity>, id: web::Path<Uuid>) -> Result<HttpResponse, ApiError> {
    if let Some(_user) = user_identity {
        // The user is authenticated
        let user = User::get(id.into_inner())?;
        Ok(HttpResponse::Ok().json(user))
    } else {
        // The user is not authenticated
        Err(ApiError::new(401, "Unauthorized".to_owned()))
    }
}

// Modify user.
#[put("/users/{id}")]
async fn update(user_identity: Option<Identity>, id: web::Path<Uuid>, user: web::Json<UserMessage>) -> Result<HttpResponse, ApiError> {
    if let Some(_user) = user_identity {
        // The user is authenticated
        let user = User::update(id.into_inner(), user.into_inner())?;
        Ok(HttpResponse::Ok().json(user))
    } else {
        // The user is not authenticated
        Err(ApiError::new(401, "Unauthorized".to_owned()))
    }
}

// Delete user.
#[delete("/users/{id}")]
async fn delete(user_identity: Option<Identity>, id: web::Path<Uuid>) -> Result<HttpResponse, ApiError> {
    if let Some(_user) = user_identity {
        // The user is authenticated
        let num_deleted = User::delete(id.into_inner())?;
        Ok(HttpResponse::Ok().json(json!({ "deleted": num_deleted })))
    } else {
        // The user is not authenticated
        Err(ApiError::new(401, "Unauthorized".to_owned()))
    }
}

// Configure API routes
pub fn users_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(find);
    cfg.service(update);
    cfg.service(delete);
}