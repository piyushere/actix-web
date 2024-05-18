use actix_web::{delete, get, patch, post, web, HttpResponse, Responder, Scope};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Todo {
    task: String,
    done: bool,
}

#[get("")]
async fn read() -> impl Responder {
    let response = web::Json(Todo {
        task: String::from("To learn actix-web"),
        done: false,
    });
    return HttpResponse::Ok().json(response);
}

#[post("")]
async fn write(todo: web::Json<Todo>) -> impl Responder {
    return todo;
}

#[delete("")]
async fn remove(todo: web::Json<Todo>) -> impl Responder {
    return todo;
}

#[patch("")]
async fn update(todo: web::Json<Todo>) -> impl Responder {
    return todo;
}

pub fn bootstrap() -> Scope {
    let scope = web::scope("todos")
        .service(read)
        .service(write)
        .service(remove)
        .service(update);
    // let guards = web::scope("protected")
    //     .guard(guard::Header("x-guard", "1"))
    //     .service(read);
    return scope;
}
