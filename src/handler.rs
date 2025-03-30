use actix_web::web;
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use sqlx::encode::IsNull::No;
use sqlx::PgPool;
use uuid::Uuid;
use crate::model::{Account, CreateAccountDto, GetAccountDto, UniversalResponse, UpdateAccountDto};



// create account
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



// get all accounts
pub async fn get_accounts(
    db_pool : web::Data<PgPool>,
) -> UniversalResponse<Option<Vec<GetAccountDto>>> {

    let all_accounts = sqlx::query!(
        "SELECT * FROM accounts"
    )
        .fetch_all(db_pool.get_ref())
        .await;

    match all_accounts {
        Ok(all_accounts) => {
            let result_accounts: Vec<GetAccountDto> = all_accounts.into_iter().map( move |row| {
                GetAccountDto{
                    user_id : row.user_id,
                    account_id : row.id,
                    balance : row.balance.to_f64().unwrap_or(0.0),
                    status : row.status
                }
            }).collect();

            UniversalResponse{
                status : "00".to_string(),
                message : "Get All Wallet Accounts".to_string(),
                data : Some(result_accounts)
            }
        }


        Err (error) => {
            println!("Error getting accounts: {:?}", error);
            UniversalResponse{
                status : "01".to_string(),
                message: "Failed to get Wallet Accounts".to_string(),
                data: None
            }
        }
    }



}




// update accounts
pub async fn update_account(
    db_pool : web::Data<PgPool>,
    request : web::Json<UpdateAccountDto>
) -> UniversalResponse<Option<GetAccountDto>>{


    let update_account_result = sqlx::query!(
        "UPDATE accounts SET status = $1 WHERE user_id = $2 RETURNING *",
        request.status,
        request.user_id
    ).fetch_one(db_pool.get_ref()).await;


    match update_account_result {
        Ok(row) => {
            println!("Updated Account : {:?}", row);
            UniversalResponse{
                status : "00".to_string(),
                message: "Succesfully Updated Account".to_string(),
                data : Some(
                    GetAccountDto{
                        user_id : row.user_id,
                        account_id : row.id,
                        status : row.status,
                        balance : row.balance.to_f64().unwrap_or(0.0),
                    }
                )
            }
        }

        Err(error) => {
            println!("Error updating account: {:?}", error);
            UniversalResponse{
                status : "01".to_string(),
                message: "Failed to update account with id : ".to_string() + &request.user_id.to_string(),
                data : None
            }
        }
    }


}




pub async fn delete_account(
    db_pool : web::Data<PgPool>,
    path : web::Path<String>
) -> UniversalResponse<Option<GetAccountDto>>{

    let user_id = path.into_inner();


    let account_exists = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM accounts WHERE user_id = $1)",
        user_id
    )
        .fetch_one(db_pool.get_ref())
        .await
        .unwrap_or(Option::from(false));

    if !account_exists.unwrap_or(false) {
        return UniversalResponse {
            status: "01".to_string(),
            message: format!("Account with user_id {} not found", user_id),
            data: None,
        };
    }

    let delete_account_result = sqlx::query!(
        "DELETE FROM accounts WHERE user_id = $1 RETURNING *",
        user_id
    )
        .fetch_optional(db_pool.get_ref()).await;


    match delete_account_result {
        Ok(row) => {
            println!("Deleted Account : {:?}", row);
            UniversalResponse{
                status : "00".to_string(),
                message: "Successfully deleted Account".to_string(),
                data : None
            }
        }

        Err(error) => {
            println!("Error deleting account: {:?}", error);
            UniversalResponse{
                status : "01".to_string(),
                message: "Failed to delete account record".to_string(),
                data: None
            }
        }

    }

}




