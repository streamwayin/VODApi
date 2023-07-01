use actix_web::{web, App, HttpResponse, HttpServer, Responder , HttpRequest};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use dotenv::dotenv;
use try_catch::catch;
use std::env;
mod gst_pipeline;

#[derive(Debug, Deserialize, Serialize)]
struct StreamRequest {
    input_url: String,
    output_urls: Vec<String>,
    loop_times: String ,

}


async fn verify_token(req: HttpRequest, _body: web::Payload) -> HttpResponse {
    // Extract the Authorization Bearer token from the request headers
    let authorization_header = req.headers().get("Authorization");
    let bearer_token = match authorization_header {
        Some(header_value) => {
            match header_value.to_str() {
                Ok(header_str) => {
                    let trimmed_token = header_str.trim();
                    if trimmed_token.starts_with("Bearer ") {
                        let token = &trimmed_token[7..];
                        token
                    } else {
                        return HttpResponse::Unauthorized().body("Invalid Authorization header");
                    }
                }
                Err(_) => {
                    return HttpResponse::Unauthorized().body("Invalid Authorization header");
                }
            }
        }
        None => {
            return HttpResponse::Unauthorized().body("Missing Authorization header");
        }
    };

    // Retrieve the token from the environment variable
    let env_token = env::var("AUTH_TOKEN").unwrap_or_default();

    // Compare the bearer_token with the env_token
    if bearer_token != env_token {
        return HttpResponse::Unauthorized().body("Invalid token");
    }

    HttpResponse::Ok()
        .content_type("application/json") // Set Content-Type header to application/json
        .finish()
}

async fn stream_handler(data: web::Json<StreamRequest> , req: HttpRequest, _body: web::Payload) -> impl Responder {

    let verification_result = verify_token(req , _body).await;
    if verification_result.status().is_success() {
    // Extract input URL and output URLs from the request
    let input_url = &data.input_url;
    let output_urls = &data.output_urls;
    let number_str = &data.loop_times;
    let loop_times:  u128 = number_str.parse().expect("Failed to parse the number");
    let mut remaining_loops = loop_times;

    // Start the streaming pipeline
    while remaining_loops > 0 {
        gst_pipeline::streaming_pipeline(input_url, output_urls);
        remaining_loops -= 1;
    }

    // Prepare the JSON response
    let response_data: Value = json!({
        "success": true,
        "message": "Streaming completed successfully",
        "loopCount": loop_times - remaining_loops,
    });

    println!("loop {}" , response_data);
    // Convert the JSON object to a string
    let response_body = serde_json::to_string(&response_data).unwrap();

    // Send the response with the JSON body
    HttpResponse::Ok().body(response_body)
} else {
    // Token is invalid, return the unauthorized response
    verification_result
}
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("VOD Server is running!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Load environment variables from the .env file
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/stream").route(web::post().to(stream_handler)))
            .service(web::resource("/").route(web::get().to(index)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
