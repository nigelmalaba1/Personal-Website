use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;
use actix_cors::Cors;

#[derive(Serialize)]
struct Project {
    name: String,
    description: String,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to my GitHub portfolio!")
}

#[get("/projects")]
async fn projects() -> impl Responder {
    let project_list = vec![
        Project {
            name: "Project 1".to_string(),
            description: "Description for Project 1".to_string(),
        },
        Project {
            name: "Project 2".to_string(),
            description: "Description for Project 2".to_string(),
        },
    ];

    HttpResponse::Ok().json(project_list)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::permissive(); // or configure as per your requirements

        App::new()
            .wrap(cors)
            .service(index)
            .service(projects)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
