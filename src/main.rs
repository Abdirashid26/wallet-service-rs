use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use sqlx::PgPool;
use dotenvy::dotenv;
use std::env;
use crate::handler::{create_account, delete_account, get_accounts, update_account, update_balance};
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
            .route("/account/all",web::get().to(get_accounts))
            .route("/account/update",web::put().to(update_account))
            .route("/account/delete/{user_id}",web::delete().to(delete_account))
            .route("/account/update/balance/{user_id}",web::patch().to(update_balance))
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