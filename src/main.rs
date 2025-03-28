use axum::{
    routing::{post, get, options},
    Router,
    Json,
};
use tower_http::cors::{CorsLayer, Any};
use std::net::SocketAddr;
use serde::{Deserialize, Serialize};
use sp1_sdk::{include_elf, ProverClient, SP1Stdin};

const ELF: &[u8] = include_elf!("truthlens");

#[derive(Deserialize)]
struct ImageRequest {
    selectedImages: String,
}

#[derive(Serialize)]
struct Response {
    score: i32,
}

async fn process(Json (images):Json<ImageRequest>) -> Json<Response> {
	
    //let mut stdin = SP1Stdin::new();
    //stdin.write(&images.selectedImages);
   
    //let client = ProverClient::from_env();
    //let (pk, _vk) = client.setup(ELF);
	
	//let proof = client.prove(&pk, &stdin).run().expect("proof generation failed");
	//let score = hex::encode(proof.public_values);
	
	let max_score = 21;
	let input = images.selectedImages ;
	let key = "011011100010110100000";
    
    let mut score :i32 = max_score;
    
    for (input_char, key_char) in input.chars().zip(key.chars()) {
        if input_char != key_char {
            score = score.saturating_sub(1);
        }
    }
    
    let input_len = input.chars().count();
    let key_len = key.chars().count();
    
    if input_len < key_len {
        score = -1
    }
	
	return Json(Response { score: score });
}

async fn ping() -> &'static str {
    "pong"
}

#[tokio::main]
async fn main() {
	let cors = CorsLayer::new()
    .allow_origin(Any)
    .allow_methods(Any)
    .allow_headers(Any);
    let app = Router::new()         
        .route("/zklens", post(process))
		.route("/zklens", options(|| async { "" }))
		.route("/ping", get(ping))
		.layer(cors);

	let port = std::env::var("PORT")
        .unwrap_or_else(|_| "10000".to_string())
        .parse::<u16>()
        .expect("PORT должен быть числом");
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("Сервер запущен на {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}