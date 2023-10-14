use cfg_if::cfg_if;

cfg_if! {
  if #[cfg(feature = "ssr")] {
    use axum::response::Response;
    use std::io::ErrorKind;
    use std::fmt::Display;
    use std::fmt;

    #[derive(Debug)]
    pub enum ServerError {
      FailedToGetDbConnection,
      InvalidUuid,
      AlreadyExists,
      InvalidIpAddr,
      FailedToFind,
      FailedToGetRealtimeServerStatus,
      InvalidPortalConfig(String)
    }

    impl Display for ServerError {
      fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
          ServerError::FailedToGetDbConnection => write!(f, "FailedToGetDbConnection"),
          ServerError::InvalidUuid => write!(f, "InvalidUuid"),
          ServerError::AlreadyExists => write!(f, "AlreadyExists"),
          ServerError::InvalidIpAddr => write!(f, "InvalidIpAddr"),
          ServerError::FailedToFind => write!(f, "FailedToFind"),
          ServerError::FailedToGetRealtimeServerStatus => write!(f, "FailedToGetRealtimeServerStatus"),
          ServerError::InvalidPortalConfig(msg) => write!(f, "InvalidPortalConfig({})", msg.clone()),
        }
      }
    }

    // impl std::error::Error for ServerError {}

    // impl sqlx::error::DatabaseError for ServerError {
    //   fn message(&self) -> &str {
    //     match self {
    //       ServerError::FailedToGetDbConnection => "Failed to get db connection",
    //       ServerError::InvalidUuid => "Invalid Uuid",
    //       ServerError::AlreadyExists => "Already exists",
    //       ServerError::InvalidIpAddr => "Invalid Ip address",
    //       ServerError::FailedToFind => "Failed to find",
    //       ServerError::FailedToGetRealtimeServerStatus => "Failed to get the realtime server status of the server, make sure the minecraft server is running on the specified port and ip and glados has access to it(can ping it).",
    //       ServerError::InvalidPortalConfig(msg) => &format!("Invalid portal config: {}", msg),
    //     }
    //   }
    //   fn kind(&self) -> sqlx::error::ErrorKind {
    //     sqlx::error::ErrorKind::CheckViolation
    //   }
    // }

    impl axum::response::IntoResponse for ServerError {
      fn into_response(self) -> Response {
        let body = match self {
          ServerError::FailedToGetDbConnection => "Failed to get db connection".to_string(),
          ServerError::InvalidUuid => "Invalid Uuid".to_string(),
          ServerError::AlreadyExists => "Already exists".to_string(),
          ServerError::InvalidIpAddr => "Invalid Ip address".to_string(),
          ServerError::FailedToFind => "Failed to find".to_string(),
          ServerError::FailedToGetRealtimeServerStatus => "Failed to get the realtime server status of the server, make sure the minecraft server is running on the specified port and ip and glados has access to it(can ping it).".to_string(),
          ServerError::InvalidPortalConfig(msg) => format!("Invalid portal config: {}", msg),
        };
        (http::StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
      }
    }
  }
}