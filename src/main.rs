use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use sqlx::PgPool;
use dotenvy::dotenv;
use std::env;
use crate::handler::create_account;
use crate::model::UniversalResponse;

mod model;
mod database;
mod handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // loads for use the env file
    let pool = database::connect().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(ping))
            .route("/account", web::post().to(create_account))
    })
        .bind("127.0.0.1:8080")?
        .run().await

}




// so the bellow is examples of a hanlder
async fn ping() -> UniversalResponse<String> {
    UniversalResponse{
        status : "00".parse().unwrap(),
        message: "Hi the Rust Wallet Service is Up".to_string(),
        data: "I can confirm it still up".to_string(),
    }
}