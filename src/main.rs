use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
mod gst_pipeline;

#[derive(Debug, Deserialize, Serialize)]
struct StreamRequest {
    input_url: String,
    output_urls: Vec<String>,
    loopTimes: u32,

}

async fn stream_handler(data: web::Json<StreamRequest>) -> impl Responder {
    // Extract input URL and output URLs from the request
    let input_url = &data.input_url;
    let output_urls = &data.output_urls;
    let loopTimes = data.loopTimes;
    let mut remaining_loops = loopTimes;

    // Start the streaming pipeline
    while remaining_loops > 0 {
        gst_pipeline::streaming_pipeline(input_url, output_urls);
        remaining_loops -= 1;
    }

    // Prepare the JSON response
    let response_data: Value = json!({
        "success": true,
        "message": "Streaming completed successfully",
        "loopCount": loopTimes - remaining_loops,
    });

    println!("loop {}" , response_data);
    // Convert the JSON object to a string
    let response_body = serde_json::to_string(&response_data).unwrap();

    // Send the response with the JSON body
    HttpResponse::Ok().body(response_body)
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("VOD Server is running!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/stream").route(web::post().to(stream_handler)))
            .service(web::resource("/").route(web::get().to(index)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
