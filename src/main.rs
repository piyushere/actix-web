use actix_web::{get, middleware, App, HttpServer, Responder};
mod routes;
use routes::todos;
use env_logger::Env;

#[get("/ping")]
async fn echo() -> impl Responder {
    return String::from("pong");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // setup logger for global level
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let app = || {
        App::new()
            .wrap(middleware::NormalizePath::trim())
            .wrap(middleware::Logger::default())
            .service(echo)
            .service(todos::bootstrap())
    };
    HttpServer::new(app).workers(1).bind(("127.0.0.1", 8000))?.run().await
}
