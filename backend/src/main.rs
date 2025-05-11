use actix_cors::Cors;
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;
use reqwest::Client;
use log::{info, error, debug};
use anyhow::Result;

// Request structure for paraphrasing API
#[derive(Deserialize)]
struct ParaphraseRequest {
    text: String,
}

// Response structure for paraphrasing API
#[derive(Serialize)]
struct ParaphraseResponse {
    paraphrased: String,
}

// Error response structure
#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

// Gemini API request structure
#[derive(Serialize, Debug)]
struct GeminiRequest {
    contents: Vec<GeminiContent>,
}

#[derive(Serialize, Debug)]
struct GeminiContent {
    parts: Vec<GeminiPart>,
}

#[derive(Serialize, Debug)]
struct GeminiPart {
    text: String,
}

// Gemini API response structure based on actual response format
#[derive(Deserialize, Debug)]
struct GeminiResponse {
    #[serde(default)]
    candidates: Vec<GeminiCandidate>,
    #[serde(default)]
    usageMetadata: Option<UsageMetadata>,
    #[serde(default)]
    modelVersion: Option<String>,
    #[serde(default)]
    error: Option<GeminiError>,
}

#[derive(Deserialize, Debug)]
struct GeminiCandidate {
    content: GeminiCandidateContent,
    #[serde(default)]
    finishReason: Option<String>,
    #[serde(default)]
    citationMetadata: Option<CitationMetadata>,
    #[serde(default)]
    avgLogprobs: Option<f64>,
}

#[derive(Deserialize, Debug)]
struct GeminiCandidateContent {
    parts: Vec<GeminiCandidatePart>,
    #[serde(default)]
    role: Option<String>,
}

#[derive(Deserialize, Debug)]
struct GeminiCandidatePart {
    text: String,
}

#[derive(Deserialize, Debug)]
struct CitationMetadata {
    #[serde(default)]
    citationSources: Vec<CitationSource>,
}

#[derive(Deserialize, Debug)]
struct CitationSource {
    startIndex: usize,
    endIndex: usize,
}

#[derive(Deserialize, Debug)]
struct UsageMetadata {
    promptTokenCount: usize,
    candidatesTokenCount: usize,
    totalTokenCount: usize,
}

#[derive(Deserialize, Debug)]
struct GeminiError {
    code: Option<i32>,
    message: Option<String>,
    status: Option<String>,
}

async fn paraphrase(req: web::Json<ParaphraseRequest>) -> impl Responder {
    let api_key = match env::var("GEMINI_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            error!("GEMINI_API_KEY environment variable not set");
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "API key not configured".to_string(),
            });
        }
    };

    // Validate input
    if req.text.trim().is_empty() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "Text cannot be empty".to_string(),
        });
    }

    // Create the prompt for paraphrasing
    let prompt = format!("Paraphrase the following text while preserving its meaning and tone. Do not add any additional text, explanations, or formatting - just return the paraphrased version:\n\n{}", req.text);

    let gemini_request = GeminiRequest {
        contents: vec![GeminiContent {
            parts: vec![GeminiPart { text: prompt }],
        }],
    };

    debug!("Sending request to Gemini API: {:?}", gemini_request);

    // Call Gemini API
    match call_gemini_api(&api_key, gemini_request).await {
        Ok(paraphrased_text) => {
            HttpResponse::Ok().json(ParaphraseResponse {
                paraphrased: paraphrased_text,
            })
        }
        Err(e) => {
            error!("Error calling Gemini API: {:?}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: format!("Failed to paraphrase text: {}", e),
            })
        }
    }
}

async fn call_gemini_api(api_key: &str, request: GeminiRequest) -> Result<String> {
    let client = Client::new();
    // Update to use the gemini-2.0-flash model as shown in the Postman example
    let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent?key={}", api_key);

    debug!("Calling Gemini API at URL: {}", url);
    
    let response = client.post(&url)
        .json(&request)
        .send()
        .await?;
    
    let status = response.status();
    let body = response.text().await?;
    
    debug!("Received response with status: {}, body: {}", status, body);
    
    // Try to parse the response as JSON
    let parsed_response: Result<GeminiResponse, serde_json::Error> = serde_json::from_str(&body);
    
    match parsed_response {
        Ok(response) => {
            // Check if there's an error in the response
            if let Some(error) = response.error {
                let error_msg = error.message.unwrap_or_else(|| "Unknown API error".to_string());
                error!("Gemini API error: {}", error_msg);
                return Err(anyhow::anyhow!("Gemini API error: {}", error_msg));
            }
            
            // Extract the response text from the first candidate
            if let Some(candidate) = response.candidates.first() {
                if let Some(part) = candidate.content.parts.first() {
                    return Ok(part.text.clone());
                }
            }
            
            Err(anyhow::anyhow!("No valid response content from Gemini API"))
        },
        Err(parse_err) => {
            error!("Failed to parse Gemini API response: {}, Response body: {}", parse_err, body);
            Err(anyhow::anyhow!("Failed to parse API response: {}", parse_err))
        }
    }
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Healthy")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    // Configure logging
    std::env::set_var("RUST_LOG", "debug,backend=debug");
    env_logger::init();

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a number");

    info!("Starting server at {}:{}", host, port);

    HttpServer::new(|| {
        // Configure CORS
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173") // Vite's default dev server port
            .allowed_origin("http://localhost:5174") // Allow alternative Vite port
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec!["Content-Type"])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .route("/api/health", web::get().to(health_check))
            .route("/api/paraphrase", web::post().to(paraphrase))
    })
    .bind((host, port))?
    .run()
    .await
}
