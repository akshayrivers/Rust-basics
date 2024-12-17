use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::fs;
use std::env;
use tokio_postgres::{NoTls, Client};

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

async fn serve_html() -> impl Responder {
    let path = env::current_dir().unwrap();
    println!("Current working directory: {}", path.display());
    
    let html_content = fs::read_to_string("src/index.html")
        .unwrap_or_else(|_| "Error loading index.html".to_string());
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html_content)
}

async fn index() -> impl Responder {
    "Html code !"
}

async fn db_connect() -> Result<Client, tokio_postgres::Error> {
    let connection_string = "postgres://postgres:mysecretpassword@localhost:5432/postgres";
    let (client, connection) = tokio_postgres::connect(connection_string, NoTls).await?;

    // Spawn a task to manage the connection
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    Ok(client)
}

#[post("/create_user")]
async fn create_user() -> impl Responder {
    let client = match db_connect().await {
        Ok(client) => client,
        Err(err) => {
            eprintln!("Failed to connect to the database: {}", err);
            return HttpResponse::InternalServerError().body("Database connection failed");
        }
    };

    // Enclose "User" in double quotes
    let query = r#"INSERT INTO "User" (email, firstname, password) VALUES ($1, $2, $3)"#;
    let firstname = "John Doe";
    let email = "john.doe@example.com";
    let password = "1123456768";

    match client.execute(query, &[&email, &firstname, &password]).await {
        Ok(_) => HttpResponse::Ok().body("User created successfully"),
        Err(err) => {
            eprintln!("Failed to execute query: {}", err);
            HttpResponse::InternalServerError().body("Failed to create user")
        }
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(application)
            .service(web::scope("/app").route("/index.html", web::get().to(index)))
            .route("/home", web::get().to(serve_html))
            .route("/hey", web::get().to(manual_hello))
            .service(create_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
