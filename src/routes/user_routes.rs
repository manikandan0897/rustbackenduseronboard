use axum::{Router, routing::get};
use crate::controllers::user_controller::get_all_users;

pub fn routes() -> Router {
    Router::new().route("/", get(get_all_users))
}
