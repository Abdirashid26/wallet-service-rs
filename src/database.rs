use std::env;
use dotenvy::dotenv;
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

/**
**
* Created by Faisal Abdirashid
* In this model class is where we will define
* all of our models and dto's for our wallet services
**/


pub  async fn connect() -> Pool<Postgres>{
    dotenv().ok(); // loads for use the env file
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");


    PgPoolOptions::new()
        .max_connections(10) // MAX 10 CONNECTIONS AT A TIME
        .connect(database_url.as_str())
        .await
        .expect("Failed to connect to database")

}