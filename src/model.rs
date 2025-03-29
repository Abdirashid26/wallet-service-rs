use actix_web::{http, HttpRequest, HttpResponse, Responder};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/**
**
* Created by Faisal Abdirashid
* In this model class is where we will define
* all of our models and dto's for our wallet services
**/


#[derive(Deserialize,Serialize,Debug,sqlx::FromRow)]
pub struct Account{
    pub id: Uuid,
    pub user_id : String,
    pub balance : f64,
    pub status : String,
    pub created_at : NaiveDateTime
}


#[derive(Deserialize,Serialize,Debug,sqlx::FromRow)]
pub struct BlockedAmount{
    pub id: String,
    pub account_id : Uuid,
    pub amount: f64,
    pub reason : Option<String>,
    pub created_at : NaiveDateTime
}


#[derive(Deserialize,Serialize,Debug)]
pub struct UniversalResponse<T : Serialize>{
    pub status : String,
    pub message : String,
    pub data : T,
}


impl <T : Serialize> Responder for UniversalResponse<T> {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::build(http::StatusCode::OK)
            .content_type("application/json")
            .json(self)
    }
}