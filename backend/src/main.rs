use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use log::info;
use backend::{AppState, health_check, paraphrase};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file if it exists
    dotenv().ok();
    
    // Initialize logging
    std::env::set_var("RUST_LOG", "debug,actix_web=info,backend=debug");
    env_logger::init();
    
    // Get API key from environment or use default
    let gemini_api_key = std::env::var("GEMINI_API_KEY")
        .unwrap_or_else(|_| "AIzaSyBNIBuwC_YA5uZwauOUp6Fa543LRYffpfc".to_string());
    
    // Create app state
    let app_state = web::Data::new(AppState {
        gemini_api_key,
    });
    
    // Configure server address
    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let server_addr = format!("{}:{}", host, port);
    
    info!("Starting server at http://{}", server_addr);
    
    // Create and run the server
    HttpServer::new(move || {
        // Configure CORS
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_origin("http://localhost:5174")
            .allowed_origin("https://ai-paraphrase-tool.vercel.app")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec!["Content-Type"])
            .max_age(3600);
        
        App::new()
            .app_data(app_state.clone())
            .wrap(cors)
            .route("/api/health", web::get().to(health_check))
            .route("/api/paraphrase", web::post().to(paraphrase))
    })
    .bind(server_addr)?
    .run()
    .await
} 