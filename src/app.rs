use axum::Router;
use crate::routes;

pub fn create_app() -> Router {
    Router::new().nest("/users", routes::user_routes::routes())
}