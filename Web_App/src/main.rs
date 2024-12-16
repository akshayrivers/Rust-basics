use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::fs;
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
#[get("/application")]
async fn application() -> impl Responder {
    HttpResponse::Ok().body("Hello world from inside the app!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
use std::env;

async fn serve_html() -> impl Responder {
    let path = env::current_dir().unwrap();
    println!("Current working directory: {}", path.display());
    
    let html_content = fs::read_to_string("index.html")
        .unwrap_or_else(|_| "Error loading index.html".to_string());
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html_content)
}


async fn index() -> impl Responder {
    "Html code !"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(application)
            .service(
                web::scope("/app")
                    .route("/index.html", web::get().to(index)),
            )
            .route("/home", web::get().to(serve_html))
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}