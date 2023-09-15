mod libs;
use libs::create_embeddings;
use serde::{Deserialize, Serialize};
use warp::{http::Method, reply::json, Filter, Rejection, Reply};

type WebResult<T> = std::result::Result<T, Rejection>;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}
#[derive(Serialize)]
pub struct EmbeddingResponse {
    pub status: String,
    pub message: String,
    pub embeddings: Vec<Vec<f32>>,
}

#[derive(Serialize, Deserialize)]
pub struct EmbeddingRequest {
    pub sentences: Vec<String>,
}

pub async fn health_checker_handler() -> WebResult<impl Reply> {
    const MESSAGE: &str = "Build Simple CRUD API with Rust";

    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };
    Ok(json(response_json))
}

pub async fn embedding_handler(body: EmbeddingRequest) -> WebResult<impl Reply> {
    const MESSAGE: &str = "Build Simple CRUD API with Rust";
    let embeddings = create_embeddings(body.sentences).expect("could not get embeddings");

    let response_json = &EmbeddingResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
        embeddings: embeddings,
    };
    Ok(json(response_json))
}

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "api=info");
    }
    pretty_env_logger::init();

    let cors = warp::cors()
        .allow_methods(&[Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        // .allow_origins(vec!["*"])
        .allow_headers(vec!["content-type"])
        .allow_credentials(true);

    let health_checker = warp::path!("api" / "healthz")
        .and(warp::get())
        .and_then(health_checker_handler);

    let embedding = warp::path!("api" / "embedding")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(embedding_handler);

    let routes = embedding
        .with(cors)
        .with(warp::log("api"))
        .or(health_checker);

    println!("🚀 Server started successfully on port 8085");
    warp::serve(routes).run(([0, 0, 0, 0], 8085)).await;
}
