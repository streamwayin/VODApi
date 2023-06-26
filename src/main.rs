use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
mod gst_pipeline;

#[derive(Debug, Deserialize, Serialize)]
struct StreamRequest {
    input_url: String,
    output_urls: Vec<String>,

}

async fn stream_handler(data: web::Json<StreamRequest>) -> impl Responder {
    // Extract input URL and output URLs from the request
    let input_url = &data.input_url;
    let output_urls = &data.output_urls;


     // Start the streaming pipeline
	 // Start the streaming pipeline
	 let result =  gst_pipeline::streaming_pipeline(input_url , output_urls) ;

    // result.into();

    HttpResponse::Ok().body("Streaming started successfully")
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
