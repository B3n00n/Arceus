pub mod health;

use axum::Router;

pub fn create_router() -> Router {
    Router::new().merge(health::routes())
}
