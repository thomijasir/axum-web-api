use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

use crate::{web, Error, Result};

#[derive(Debug, Deserialize)]
struct LoginPayload {
  username: String,
  password: String,
}

pub fn routes() -> Router {
  Router::new().route("/api/login", post(api_login))
}

async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
  println!("->> {:<12} - api_login - {payload:?}", "HANDLER");

  if payload.username != "demo1" || payload.password != "welcome" {
    return Err(Error::LoginFail);
  }

  // FIXME: implement real auth from DB
  cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));

  let body = Json(json!({
    "result": {
      "success": true,
    }
  }));

  Ok(body)
}
