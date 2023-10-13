use cfg_if::cfg_if;

cfg_if! {
  if #[cfg(feature = "ssr")] {
    use sqlx::sqlite::SqliteConnection;
    use sqlx::Connection;
    use axum::Json;
    use axum::extract::{Path, Query};
    use axum::response::IntoResponse;
    use axum::response::Response;
    use log::{debug, info, error};
    use serde_json::json;
    use crate::api::schema::{PaginatedResult, Server};
    use serde::Deserialize;
    use crate::api::error::ServerError;
    use std::net::Ipv4Addr;

    #[derive(Deserialize)]
    pub struct Pagination {
        page: u32,
        per_page: u32,
    }

    impl Default for Pagination {
        fn default() -> Self {
            Self { page: 0, per_page: 10 }
        }
    }

    pub async fn db_setup() -> anyhow::Result<()> {
      let mut conn = db().await?;

      sqlx::query(r#"CREATE TABLE IF NOT EXISTS servers (
        id TEXT(36) NOT NULL,
        ip BINARY(4) NOT NULL,
        port SMALLINT UNSIGNED NOT NULL,
        name VARCHAR(32) NOT NULL,
        PRIMARY KEY (id)
       );"#).execute(&mut conn).await?;

      Ok(())
    }

    async  fn db() -> anyhow::Result<SqliteConnection> {
      let conn = SqliteConnection::connect("glados.db?mode=rwc").await?;
      Ok(conn)
    }

    pub async fn delete_unregister_server(Path(id): Path<String>) -> Result<http::StatusCode, ServerError> {
      info!("Delete /server/{}", id);
      let mut db_conn = db().await.map_err(|_| -> ServerError {ServerError::FailedToGetDbConnection})?;
      let delete_query = sqlx::query("delete from servers where id=?").bind(&id);
      if delete_query.execute(&mut db_conn).await.is_ok() {
        return Ok(http::StatusCode::OK);
      }

      return Err(ServerError::FailedToFind);
    }

    pub async fn put_json_update_server(Path(id): Path<String>, Json(server): Json<Server>) -> Result<Json<Server>, ServerError> {
      info!("PUT /server/{}: Registering new server {{ id: {}, ip: {}, port: {}, name: {} }}", id, server.id, server.ip, server.port, server.name);
      let mut db_conn = db().await.map_err(|_| -> ServerError {ServerError::FailedToGetDbConnection})?;
      let put_query = sqlx::query("update servers set id=?, name=?, port=?, ip=? where id=?").bind(server.id.to_string()).bind(&server.name).bind(server.port).bind(Into::<u32>::into(server.ip)).bind(id);
      if put_query.execute(&mut db_conn).await.is_ok() {
        return Ok(Json(server));
      }

      return Err(ServerError::AlreadyExists);
    }

    pub async fn get_server_by_id(Path(id): Path<String>) -> Result<Json<Server>, ServerError> {
      info!("GET /server/{}", id);
      let mut db_conn = db().await.map_err(|_| -> ServerError {ServerError::FailedToGetDbConnection})?;
      let get_query = sqlx::query_as::<_, Server>("select * from servers where id = ?").bind(&id);
      Ok(Json(get_query.fetch_one(&mut db_conn).await.map_err(|_| -> ServerError {ServerError::FailedToFind})?))
    }

    pub async fn post_json_register_server(Json(server): Json<Server>) -> Result<Json<Server>, ServerError> {
      info!("POST /server: Registering new server {{ id: {}, ip: {}, port: {}, name: {} }}", server.id, server.ip, server.port, server.name);
      let mut db_conn = db().await.map_err(|_| -> ServerError {ServerError::FailedToGetDbConnection})?;
      let insert_query = sqlx::query("insert into servers (id, ip, port, name) values (?, ?, ?, ?)").bind(server.id.to_string()).bind(Into::<u32>::into(server.ip)).bind(server.port).bind(server.name.clone());
      if insert_query.execute(&mut db_conn).await.is_ok() {
        return Ok(Json(server));
      }

      return Err(ServerError::AlreadyExists);
    }

    pub async fn get_list_servers(pagination: Option<Query<Pagination>>) -> impl IntoResponse {
      let Query(pagination) = pagination.unwrap_or_default();
      let offset = pagination.page * pagination.per_page;

      info!("GET /server: Getting list of servers with offset {} fetching {} rows.", offset, pagination.per_page);

      if let Ok(mut db_conn) = db().await {
        let select_query = sqlx::query_as::<_, Server>("SELECT * FROM servers LIMIT ? OFFSET ?").bind(pagination.per_page).bind(offset);
        if let Ok(servers) = select_query.fetch_all(&mut db_conn).await {
          return Json(json!(servers));
        }
      }

      let json = json!(Vec::<Server>::new());
      return Json(json);
    }
  }
}