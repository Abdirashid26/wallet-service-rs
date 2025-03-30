use actix_web::web;
use sqlx::PgPool;
use uuid::Uuid;
use crate::model::{CreateAccountDto, GetAccountDto, UniversalResponse};

pub async fn create_account(
    db_pool : web::Data<PgPool>,
    create_account_dto: web::Json<CreateAccountDto>
) -> UniversalResponse<String>{


    let my_uuid = Uuid::new_v4();

    let create_account_result = sqlx::query!(
        "INSERT INTO accounts (id, user_id, balance, status) VALUES ($1, $2, $3, $4)",
        my_uuid,
        create_account_dto.user_id,
        0.0 as f64,
        "true",
    ).execute(db_pool.get_ref())
    .await;


    match create_account_result {
        Ok(rows_affected) => {
            println!("Created Account : {:?}", rows_affected);
            UniversalResponse{
                status : "00".parse().unwrap(),
                message : "Created a new account Successfully".parse().unwrap(),
                data : "".to_string()
            }
        }
        Err(error) => {
            println!("Error creating account: {:?}", error);
            UniversalResponse{
                status: "01".to_string(),
                message : "Failed to Create Account".to_string(),
                data: "".to_string()
            }
        }
    }


}




pub async fn get_accounts(
    db_pool : web::Data<PgPool>
) -> UniversalResponse<Vec<GetAccountDto>> {






}












