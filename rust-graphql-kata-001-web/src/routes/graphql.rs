use actix_web::{web, HttpResponse, Result};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::post().to(index))
        .route("/", web::get().to(index_playground));
}

async fn index() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn index_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
        )))
}
