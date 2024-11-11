use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
};

pub type Result<T> = std::result::Result<T, Error>;
#[derive(Debug)]
pub enum Error {
  LoginFail,
  AuthFailNoAuthTokenCookie,
  AuthFailTokenWrongFormat,
  TicketDeleteFailIdNotFound { id: u64 },
}

impl IntoResponse for Error {
  fn into_response(self) -> Response {
    println!("->> {:<12} - {self:?}", "INTO_RES");
    (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
  }
}

impl std::fmt::Display for Error {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> core::result::Result<(), std::fmt::Error> {
    write!(fmt, "{self:?}")
  }
}
