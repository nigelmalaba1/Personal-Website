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
            name: "Serverless".to_string(),
            description: "Serverless Architecture that uses AWS lambda and reads data from DynamoDB and writes to AWS S3 bucket".to_string(),
        },
        Project {
            name: "Machine Learning with Azure Databricks".to_string(),
            description: "Heart Attack Prediction tool that uses Azure Databricks AutoML to train a model using heart prediction analysis data from Kaggle. Built a fast API service and deployed it on AWS ECR. Service is accessible via AWS App Runner hosted domain ".to_string(),
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
