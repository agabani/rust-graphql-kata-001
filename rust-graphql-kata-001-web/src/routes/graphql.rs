use crate::graphql::GraphQLSchema;
use actix_web::{web, HttpResponse};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::Request;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::post().to(index))
        .route("/", web::get().to(index_playground));
}

async fn index(schema: web::Data<GraphQLSchema>, request: web::Json<Request>) -> HttpResponse {
    HttpResponse::Ok().json(schema.execute(request.0).await)
}

async fn index_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/graphql/").subscription_endpoint("/graphql/"),
        ))
}
