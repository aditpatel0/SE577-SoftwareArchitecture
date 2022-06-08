use std::sync::Arc;
use axum::{
    extract::{Extension, Path, Query},
    http::{HeaderValue, Method},
    routing::get,
    response::Json,
    Router,
};
use tower_http::cors::CorsLayer;
use serde_json;
use serde_yaml;
use reqwest::Client;
use std::collections::HashMap;

#[tokio::main]
async fn main() {

    let content = get_repositories_from_yaml_file();

    let shared_http_client = Arc::new(Client::new());

    let app = Router::new().route("/fake_repositories", get(|| async { Json(content) }))
                           .route("/user", get(get_user))
                           .route("/repositories", get(get_repositories))
                           .route("/:repo/branches", get(get_branches_for_repo))
                           .route("/gists", get(get_gists))
                           .route("/gists/:gist_id", get(get_gist))
                           .layer(Extension(shared_http_client))
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

async fn get_gists(Extension(http_client): Extension<Arc<Client>>) -> String {
    return http_client.get(format!("https://api.github.com/users/apatel578/gists"))
                      .header("Accept", "application/vnd.github.v3+json")
                      .header("User-Agent", "Test")
                      .send()
                      .await
                      .unwrap()
                      .text()
                      .await
                      .unwrap();
}

async fn get_gist(Extension(http_client): Extension<Arc<Client>>, Path(gist_id): Path<String>) -> String {
    return http_client.get(format!("https://api.github.com/gists/{gist_id}"))
                      .header("Accept", "application/vnd.github.v3+json")
                      .header("User-Agent", "Test")
                      .send()
                      .await
                      .unwrap()
                      .text()
                      .await
                      .unwrap();
}

async fn get_user(Extension(http_client): Extension<Arc<Client>>, Query(params): Query<HashMap<String, String>>) -> String {
    if params.contains_key("authenticated") && params["authenticated"].eq_ignore_ascii_case("true") {
        let pat: String = match std::env::var("GH_PAT") {
            Ok(pat) => pat,
            Err(_error) => "".to_string(),
        };
        let auth_value = base64::encode(format!("apatel578:{pat}"));
        return http_client.get("https://api.github.com/user")
                          .header("Authorization", format!("Basic {auth_value}"))
                          .header("Accept", "application/vnd.github.v3+json")
                          .header("User-Agent", "Test")
                          .send()
                          .await
                          .unwrap()
                          .text()
                          .await
                          .unwrap();
    } else {
        return http_client.get("https://api.github.com/users/apatel578")
                          .header("Accept", "application/vnd.github.v3+json")
                          .header("User-Agent", "Test")
                          .send()
                          .await
                          .unwrap()
                          .text()
                          .await
                          .unwrap();
    }
}

async fn get_repositories(Extension(http_client): Extension<Arc<Client>>, Query(params): Query<HashMap<String, String>>) -> String {
    if params.contains_key("authenticated") && params["authenticated"].eq_ignore_ascii_case("true") {
        let pat: String = match std::env::var("GH_PAT") {
            Ok(pat) => pat,
            Err(_error) => "".to_string(),
        };
        let auth_value = base64::encode(format!("apatel578:{pat}"));
        return http_client.get("https://api.github.com/user/repos")
                          .header("Authorization", format!("Basic {auth_value}"))
                          .header("Accept", "application/vnd.github.v3+json")
                          .header("User-Agent", "Test")
                          .send()
                          .await
                          .unwrap()
                          .text()
                          .await
                          .unwrap();
    } else {
        return http_client.get("https://api.github.com/users/apatel578/repos")
                          .header("Accept", "application/vnd.github.v3+json")
                          .header("User-Agent", "Test")
                          .send()
                          .await
                          .unwrap()
                          .text()
                          .await
                          .unwrap();
    }
}

async fn get_branches_for_repo(Extension(http_client): Extension<Arc<Client>>, Path(repo): Path<String>) -> String {
    return http_client.get(format!("https://api.github.com/repos/apatel578/{repo}/branches"))
                      .header("Accept", "application/vnd.github.v3+json")
                      .header("User-Agent", " Test")
                      .send()
                      .await
                      .unwrap()
                      .text()
                      .await
                      .unwrap();
}

fn get_repositories_from_yaml_file() -> serde_json::Value {
    let args: Vec<String> = std::env::args().collect();
    let path: String = args[1].clone();
    let file_handle = std::fs::File::open(path).unwrap();
    let contents: serde_json::Value = serde_yaml::from_reader(file_handle).unwrap();
    return contents;
