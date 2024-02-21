mod auth;

use actix_web::web::ServiceConfig;
use auth::auth_views_factory;

pub fn view_factory(app: &mut ServiceConfig) {
    auth_views_factory(app);
}
