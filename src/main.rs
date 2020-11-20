use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use askama::Template;
use serde::Deserialize;

#[derive(Template)]
#[template(path = "authorize.html")]
struct AuthorizeTemplate<'a> {
    client_id: &'a str,
}

#[derive(Deserialize)]
struct AuthorizeQuery {
    response_type: String,
    client_id: String,
    redirect_uri: String,
}

#[get("/authorize")]
async fn authorize(params: web::Query<AuthorizeQuery>) -> impl Responder {
    // TODO: check if `response_type = "code"`
    println!("{}", params.client_id);

    // TODO: check if `client_id` matches

    let template = AuthorizeTemplate {
        client_id: &params.client_id,
    };
    HttpResponse::Ok().body(template.render().unwrap())
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(authorize)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
