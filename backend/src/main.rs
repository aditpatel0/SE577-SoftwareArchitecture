use axum::{
    http::{HeaderValue, Method},
    routing::get,
    response::Json,
    Router,
};
use tower_http::cors::CorsLayer;
use serde_json;
use serde_yaml;

#[tokio::main]
async fn main() {

    let path = get_file_path_from_first_arg();

    let content = get_repositories_from_yaml_file(path);

    let app = Router::new().route("/repositories", get(|| async { Json(content) }))
                           .layer(
                               CorsLayer::new()
                                   .allow_origin("http://localhost:8080".parse::<HeaderValue>().unwrap())
                                   .allow_methods(vec![Method::GET]),
                           );

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn get_file_path_from_first_arg() -> String {
    let args: Vec<String> = std::env::args().collect();
    return args[1].clone();
}

fn get_repositories_from_yaml_file(path: String) -> serde_json::Value {
    println!("Opening: {}", &path);
    let file_handle = std::fs::File::open(path).unwrap();
    let contents: serde_json::Value = serde_yaml::from_reader(file_handle).unwrap();
    return contents;
}
