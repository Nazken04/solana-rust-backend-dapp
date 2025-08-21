use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use solana_client::rpc_client::RpcClient;
use std::str::FromStr;
use dotenv::dotenv;
use std::env;
use serde_json::json;

#[derive(Serialize, Deserialize)]
struct StakeRequest {
    user_public_key: String,
    amount: u64,
}

#[derive(Serialize, Deserialize)]
struct UnstakeRequest {
    user_public_key: String,
    amount: u64,
}

#[derive(Serialize, Deserialize)]
struct BalanceResponse {
    balance: u64,
}

async fn stake(req: web::Json<StakeRequest>) -> impl Responder {
    let _rpc_url = env::var("SOLANA_RPC_URL").expect("SOLANA_RPC_URL must be set");
    let _program_id_str = env::var("STAKING_PROGRAM_ID").expect("STAKING_PROGRAM_ID must be set");

    HttpResponse::Ok().json(json!({
        "status": "success",
        "message": format!("Staked {} tokens for user {}", req.amount, req.user_public_key)
    }))
}

async fn unstake(req: web::Json<UnstakeRequest>) -> impl Responder {
     HttpResponse::Ok().json(json!({
        "status": "success",
        "message": format!("Unstaked {} tokens for user {}", req.amount, req.user_public_key)
    }))
}


async fn balance(user_public_key: web::Path<String>) -> impl Responder {
    match web::block(move || {
        let rpc_url = env::var("SOLANA_RPC_URL").expect("SOLANA_RPC_URL must be set");
        let client = RpcClient::new(rpc_url);
        let user_pubkey = Pubkey::from_str(&user_public_key).unwrap();
        client.get_balance(&user_pubkey)
    }).await {
        Ok(Ok(balance)) => HttpResponse::Ok().json(BalanceResponse { balance }), 
        _ => HttpResponse::InternalServerError().json(json!({"error": "Failed to fetch balance"})), 
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    println!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .route("/stake", web::post().to(stake))
            .route("/unstake", web::post().to(unstake))
            .route("/balance/{user_public_key}", web::get().to(balance))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
