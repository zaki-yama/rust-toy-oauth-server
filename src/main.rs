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

#[derive(Deserialize, Debug)]
struct DecisionFormData {
    approve: Option<bool>,
    deny: Option<bool>,
}

#[post("/decision")]
async fn decision(req_body: web::Form<DecisionFormData>) -> impl Responder {
    match req_body.approve {
        Some(true) => println!("approved"),
        _ => {}
    }
    println!("{:?}", req_body);
    HttpResponse::Ok().body("hello")
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(authorize)
            .service(decision)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
