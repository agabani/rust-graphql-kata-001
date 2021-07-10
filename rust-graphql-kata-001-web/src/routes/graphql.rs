use crate::database::Database;
use crate::domain::UserId;
use crate::graphql::GraphQLSchema;
use actix_web::{web, HttpResponse};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::Request;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::post().to(index))
        .route("/", web::get().to(index_playground));
}

async fn index(
    database: web::Data<Database>,
    schema: web::Data<GraphQLSchema>,
    http_request: web::HttpRequest,
    request: web::Json<Request>,
) -> HttpResponse {
    let mut request = request.0.data(database);

    if let Some(user_id) = http_request
        .headers()
        .get("user-id")
        .and_then(|value| value.to_str().map(|value| UserId(value.to_string())).ok())
    {
        request = request.data(user_id);
    }

    let response = schema.execute(request).await;

    HttpResponse::Ok().json(response)
}

async fn index_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/graphql/").subscription_endpoint("/graphql/"),
        ))
}
