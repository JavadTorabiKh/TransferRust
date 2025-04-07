use std::collections::HashMap;
use actix_web::{web, Responder, HttpResponse};
use sui_sdk::types::base_types::SuiAddress;
use std::str::FromStr;
use sui_sdk::{ SuiClientBuilder};
use serde::Serialize;
use num_traits::pow::Pow;
use config::Config;
use actix_web::HttpRequest;
mod config;



#[derive(Serialize)]
struct CustomResponse {
    success: bool,
    message: String,
    data: Option<f64>, // Changed type to f64 for decimal data
}

fn to_token(amount: f64, decimal: f32) -> f64 {
    let base: f32 = 10.0;
    let floated = amount / base.powf(decimal.into()) as f64;
    floated
}

pub async fn get_balance(req: HttpRequest, address: web::Path<String>) -> impl Responder {
    let active_address = SuiAddress::from_str(&address);
    let my_config = Config::new();
    let headers = req.headers();
    let mut token = "";
    for (name, value) in headers.iter() {
        if name.as_str() == "authorization"{
            token = value.to_str().unwrap_or("Invalid UTF-8");
        }
    }
    if (token == my_config.bearer.token){
        match active_address {
            Ok(active_address) => {
                let sui_network = SuiClientBuilder::default().build_devnet().await;        
                match sui_network {
                    Ok(sui_network) => {
                        let total_balance = sui_network
                            .coin_read_api()
                            .get_all_balances(active_address)
                            .await;

                        match total_balance {
                            Ok(total_balance) => {
                                for item in total_balance {
                                    if item.coin_type == "0x2::sui::SUI" {
                                        let amount=item.total_balance as f64;
                                        let dec = my_config.sui.decimal;
                                        let divided_balance = to_token(amount,dec);
                                        let response = CustomResponse {
                                            success: true,
                                            message: "".to_string(),
                                            data: Some(divided_balance),
                                        };

                                        return HttpResponse::Ok().json(response);
                                    }
                                }

                                let response = CustomResponse {
                                    success: true,
                                    message: "No balance found for the specified coin type.".to_string(),
                                    data: None,
                                };
                                return HttpResponse::Ok().json(response);
                            },
                            Err(err) => {
                                eprintln!("Error: {}", err);
                                let response = CustomResponse {
                                    success: false,
                                    message: "Internal Server Error".to_string(),
                                    data: None,
                                };
                                return HttpResponse::InternalServerError().json(response);
                            }
                        }
                    },
                    Err(err) => {
                        eprintln!("Error: {}", err);
                        let response = CustomResponse {
                            success: false,
                            message: "Internal Server Error".to_string(),
                            data: None,
                        };
                        return HttpResponse::InternalServerError().json(response);
                    }
                }
            },
            Err(err) => {
                eprintln!("Error: {}", err);
                let response = CustomResponse {
                    success: false,
                    message: "Invalid address".to_string(),
                    data: None,
                };
                return HttpResponse::BadRequest().json(response);
            }
        }
    }else{
        let response = CustomResponse {
                            success: false,
                            message: "UnAuthorization".to_string(),
                            data: None,
                        };
        return HttpResponse::InternalServerError().json(response);
    }
}

