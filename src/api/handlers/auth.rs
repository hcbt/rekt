use crate::api_error::ApiError;
use crate::api::models::users::{User, UserMessage};
use actix_web::{post, web, HttpRequest, HttpResponse, HttpMessage};
use actix_identity::Identity;
use serde_json::json;

// Register an user.
#[post("/register")]
async fn register(user: web::Json<UserMessage>) -> Result<HttpResponse, ApiError> {
    let user = User::create(user.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}

// Sign in.
#[post("/sign-in")]
async fn sign_in(request: HttpRequest, credentials: web::Json<UserMessage>) -> Result<HttpResponse, ApiError> {
    let credentials = credentials.into_inner();

    // Find user by email
    let user = User::get_by_email(credentials.email)
        .map_err(|e| {
            match e.status_code {
                404 => ApiError::new(401, "Credentials not valid!".to_string()),
                _ => e,
            }
        })?;
    
    // Verify password
    let is_valid = user.verify_password(credentials.password.as_bytes())?;

    if is_valid == true {
        // attach a verified user identity to the active session
        let user_id = user.id.to_string();
        Identity::login(&request.extensions_mut(), user_id).unwrap();
        Ok(HttpResponse::Ok().json(user))
    } else {
        Err(ApiError::new(401, "Credentials not valid!".to_string()))
    }
}

// Sign out.
#[post("/sign-out")]
async fn sign_out(user: Identity) -> Result<HttpResponse, ApiError> {
    user.logout();
    Ok(HttpResponse::Ok().json(json!({ "message": "Successfully signed out" })))
}

// Register all routes.
pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(register);
    cfg.service(sign_in);
    cfg.service(sign_out);
}