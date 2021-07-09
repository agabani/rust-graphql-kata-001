use actix_web::{web, HttpResponse};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::post().to(index))
        .route("/", web::get().to(index_playground));
}

async fn index() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn index_playground() -> HttpResponse {
    HttpResponse::Ok().finish()
}
