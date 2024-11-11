#![allow(unused)]

use std::net::SocketAddr;

use axum::{
  extract::{Path, Query},
  middleware,
  response::{Html, IntoResponse, Response},
  routing::{get, get_service},
  Router,
};
use model::ModelController;
use serde::Deserialize;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

pub use self::error::{Error, Result};

mod ctx;
mod error;
mod model;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
  // Initialize ModelController
  let mc = ModelController::new().await?;

  // Add APIs Tickets
  let routes_apis = web::routes_tickets::routes(mc.clone())
    .route_layer(middleware::from_fn(web::middleware_auth::mw_require_auth));

  // router register
  let routes_all = Router::new()
    .merge(routes_hello())
    .merge(web::routes_login::routes())
    .nest("/api", routes_apis)
    .layer(middleware::map_response(main_response_mapper))
    .layer(CookieManagerLayer::new())
    .fallback_service(routes_static());
  // region start server
  let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
  println!("->> LISTENING on {addr}\n");
  axum::Server::bind(&addr)
    .serve(routes_all.into_make_service())
    .await
    .unwrap();

  Ok(())
}

async fn main_response_mapper(res: Response) -> Response {
  println!("->> {:<12} - main_response_mapper - {res:?}", "RES_MAPPER");
  println!();
  res
}

fn routes_static() -> Router {
  Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

fn routes_hello() -> Router {
  Router::new()
    .route("/hello", get(handler_hello))
    .route("/hello/:name", get(handler_hello2))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
  name: Option<String>,
}

// hello with Query params hello?name=World
async fn handler_hello(params: Query<HelloParams>) -> impl IntoResponse {
  println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");
  let name = params.name.as_deref().unwrap_or("World");
  Html(format!("Hello, <b>{name}</b>"))
}

// hello with param path /hello/:name
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
  println!("->> {:<12} - handler_hello2 - {name}", "HANDLER");
  Html(format!("Hello <b>{name}</b>"))
}
