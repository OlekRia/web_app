mod create;
mod edit;
mod get;

use actix_web::web::{get, post, scope, ServiceConfig};
use get as my_get;

pub fn to_do_views_factory(app: &mut ServiceConfig) {
    app.service(
        scope("v1/item")
            .route("create/{title}", post().to(create::create))
            .route("get", get().to(my_get::get))
            .route("edit", post().to(edit::edit)),
    );
}
