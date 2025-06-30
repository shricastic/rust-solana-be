use actix_web::{get, web, HttpResponse, Responder};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

const RPC_URL: &str = "https://api.devnet.solana.com";

#[get("/solana/balance/{pubkey}")]
pub async fn get_balance(path: web::Path<String>) -> impl Responder {
    let pubkey = Pubkey::from_str(&path.into_inner());

    if let Err(_) = pubkey {
        return HttpResponse::BadRequest().body("Invalid pubkey");
    }

    let client = RpcClient::new(RPC_URL.to_string());

    match client.get_balance(&pubkey.unwrap()).await {
        Ok(balance) => HttpResponse::Ok().json(serde_json::json!({
            "balance_sol": balance as f64 / 1_000_000_000.0
        })),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/solana/account-info/{pubkey}")]
pub async fn account(path: web::Path<String>) -> impl Responder {
    let pubkey = Pubkey::from_str(&path.into_inner());
    if pubkey.is_err() {
        return HttpResponse::BadRequest().body("Invalid pubkey");
    }
    let client = RpcClient::new(RPC_URL.to_string());

    match client.get_account(&pubkey.unwrap()).await {
        Ok(account) => HttpResponse::Ok().json(serde_json::json!({
            "lamports": account.lamports,
            "owner": account.owner.to_string(),
            "data_len": account.data.len(),
            "executable": account.executable
        })),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/solana/program-accounts/{program_id}")]
pub async fn program_accounts(path: web::Path<String>) -> impl Responder {
    let program_id = Pubkey::from_str(&path.into_inner());
    if program_id.is_err() {
        return HttpResponse::BadRequest().body("Invalid program ID");
    }
    let client = RpcClient::new(RPC_URL.to_string());

    match client.get_program_accounts(&program_id.unwrap()).await {
        Ok(accounts) => {
            let keys: Vec<_> = accounts.into_iter().map(|(k, _)| k.to_string()).collect();
            HttpResponse::Ok().json(keys)
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
