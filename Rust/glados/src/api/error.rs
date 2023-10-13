use cfg_if::cfg_if;

cfg_if! {
  if #[cfg(feature = "ssr")] {
    use axum::response::Response;

    pub enum ServerError {
      FailedToGetDbConnection,
      InvalidUuid,
      AlreadyExists,
      InvalidIpAddr,
      FailedToFind
    }

    impl axum::response::IntoResponse for ServerError {
      fn into_response(self) -> Response {
        let body = match self {
          ServerError::FailedToGetDbConnection => "Failed to get db connection",
          ServerError::InvalidUuid => "Invalid Uuid",
          ServerError::AlreadyExists => "Already exists",
          ServerError::InvalidIpAddr => "Invalid Ip address",
          ServerError::FailedToFind => "Failed to find",
        };
        (http::StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
      }
    }
  }
}