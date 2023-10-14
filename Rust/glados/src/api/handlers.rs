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
    use crate::api::schema::{PaginatedResult, Server, ServerStatus};
    use serde::Deserialize;
    use crate::api::error::ServerError;
    use std::net::Ipv4Addr;
    use super::schema::PortalConfig;
    use crate::api::schema::Portal;

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

      sqlx::query(r#"create table if not exists servers (
        id text(36) not null,
        ip binary(4) not null,
        port smallint unsigned not null,
        name varchar(32) not null,
        primary key (id)
       );"#).execute(&mut conn).await?;

      sqlx::query(r#"create table if not exists server_status (
        timestamp text not null,
        server_id text(36),
        online int unsigned not null,
        num_players int unsigned not null,
        max_players int unsigned not null,
        portals int unsigned not null
      );"#).execute(&mut conn).await?;

      sqlx::query(r#"create table if not exists server_overview (
        timestamp text not null,
        server_id text(36),
        online int unsigned not null,
        servers int unsigned not null,
        portals int unsigned not null
      );"#).execute(&mut conn).await?;

      sqlx::query(r#"create table if not exists portal_config (
        frame_block_id text not null,
        name text not null,
        id text(36) not null,
        color binary(3) not null,
        ignite_with text not null,
        ignite_with_id text,
        primary key (id)
      )"#).execute(&mut conn).await?;

      sqlx::query(r#"create table if not exists portals (
        id text(36) not null,
        name text,
        host text(36),
        pos_x integer not null,
        pos_y integer not null,
        pos_z integer not null,
        primary key (id)
      )"#).execute(&mut conn).await?;

      Ok(())
    }

    async fn db() -> anyhow::Result<SqliteConnection> {
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

    pub async fn get_get_server_status(Path(id): Path<String>) -> Result<Json<mc_query::status::StatusResponse>, ServerError> {
      info!("GET /server/status/{} Getting server status", id);
      let data = mc_query::status("10.0.2.4", 25565).await.map_err(|_| -> ServerError {ServerError::FailedToGetRealtimeServerStatus})?;
      info!("data {data:#?}");
      Ok(Json(data))
    }

    // Portal Config api

    pub async fn get_list_portal_configs(pagination: Option<Query<Pagination>>) -> Result<Json<Vec<PortalConfig>>, ServerError> {
      let Query(pagination) = pagination.unwrap_or_default();
      let offset = pagination.page * pagination.per_page;
      info!("GET /portal/config: Getting {} portal configs offset at {}", pagination.per_page, offset);

      let mut db_conn = db().await.map_err(|_| -> ServerError {ServerError::FailedToGetDbConnection})?;
      let get_query = sqlx::query_as::<_, PortalConfig>("select * from portal_config limit ? offset ?").bind(pagination.per_page).bind(offset);
      let portal_configs = get_query.fetch_all(&mut db_conn).await.map_err(|ServerError| -> ServerError {ServerError::FailedToFind})?;

      Ok(Json(portal_configs))
    }

    pub async fn post_create_portal_config(Json(config): Json<PortalConfig>) -> Result<Json<PortalConfig>, ServerError> {
      info!("POST /portal/config: {:?}", config);
      let mut db_conn = db().await.map_err(|_| -> ServerError {ServerError::FailedToGetDbConnection})?;
      let color = (config.color.red as i32) << 16 | (config.color.green as i32) << 8 | config.color.blue as i32;
      let (ignite_with, ignite_with_id) = config.ignite_with.deconstruct();
      let post_query = sqlx::query("insert into portal_config (id, name, frame_block_id, color, ignite_with, ignite_with_id) values (?, ?, ?, ?, ?, ?)").bind(config.id.to_string()).bind(config.name.clone()).bind(config.frame_block_id.clone()).bind(color).bind(ignite_with).bind(ignite_with_id);
      if post_query.execute(&mut db_conn).await.is_ok() {
        return Ok(Json(config));
      } else {
        return Err(ServerError::AlreadyExists);
      }
    }

    pub async fn get_portal_config(Path(id): Path<String>) -> Result<Json<PortalConfig>, ServerError> {
      info!("GET /portal/config/{}", id.clone());
      let mut db_conn = db().await.map_err(|_| -> ServerError {ServerError::FailedToGetDbConnection})?;
      let get_query = sqlx::query_as::<_, PortalConfig>("select * from portal_config where id = ?").bind(id);
      let config = get_query.fetch_one(&mut db_conn).await.map_err(|_| -> ServerError {ServerError::FailedToFind})?;
      Ok(Json(config))
    }

    pub async fn put_update_portal_config(Path(id): Path<String>, Json(config): Json<PortalConfig>) -> Result<Json<PortalConfig>, ServerError> {
      info!("PUT /portal/config/{}: Updating portal config to {:?}", id, config);
      let mut db_conn = db().await.map_err(|_| -> ServerError {ServerError::FailedToGetDbConnection})?;
      let color = (config.color.red as i32) << 16 | (config.color.green as i32) << 8 | (config.color.blue as i32);
      let (ignite_with, ignite_with_id) = config.ignite_with.deconstruct();
      let put_query = sqlx::query("update portal_config set id=?, name=?, frame_block_id=?, color=?, ignite_with=?, ignite_with_id=? where id=?").bind(config.id.to_string()).bind(config.name.clone()).bind(config.frame_block_id.clone()).bind(color).bind(ignite_with).bind(ignite_with_id).bind(id.to_string());
      if put_query.execute(&mut db_conn).await.is_ok() {
        return Ok(Json(config));
      }

      return Err(ServerError::AlreadyExists);
    }

    pub async fn delete_portal_config(Path(id): Path<String>) -> Result<http::StatusCode, ServerError> {
      info!("DELETE /portal/config/{}", id);
      let mut db_conn = db().await.map_err(|_| -> ServerError {ServerError::FailedToGetDbConnection})?;
      let delete_query = sqlx::query("delete from portal_config where id=?").bind(&id);
      if delete_query.execute(&mut db_conn).await.is_ok() {
        return Ok(http::StatusCode::OK);
      }

      return Err(ServerError::FailedToFind);
    }

    // Portals

    pub async fn get_list_portals(pagination: Option<Query<Pagination>>) -> Result<Json<Vec<Portal>>, ServerError> {
      let Query(pagination) = pagination.unwrap_or_default();
      let offset = pagination.page * pagination.per_page;
      info!("GET /portal: Getting {} portals offset at {}", pagination.per_page, offset);

      let mut db_conn = db().await.map_err(|_| -> ServerError {ServerError::FailedToGetDbConnection})?;
      let get_query = sqlx::query_as::<_, Portal>("select * from portals limit ? offset ?").bind(pagination.per_page).bind(offset);
      let portals = get_query.fetch_all(&mut db_conn).await.map_err(|ServerError| -> ServerError {ServerError::FailedToFind})?;

      Ok(Json(portals))
    }

    pub async fn post_create_portal(Json(portal): Json<Portal>) -> Result<Json<Portal>, ServerError> {
      info!("POST /portal: {:?}", portal);
      let mut db_conn = db().await.map_err(|_| -> ServerError {ServerError::FailedToGetDbConnection})?;
      let post_query = sqlx::query("insert into portals (id, name, host, pos_x, pos_z, pos_y) values (?, ?, ?, ?, ?, ?)").bind(portal.id.to_string()).bind(portal.name.clone()).bind(portal.host.to_string()).bind(portal.pos_x).bind(portal.pos_x).bind(portal.pos_z).bind(portal.pos_y);
      if post_query.execute(&mut db_conn).await.is_ok() {
        return Ok(Json(portal));
      } else {
        return Err(ServerError::AlreadyExists);
      }
    }

    pub async fn get_portal(Path(id): Path<String>) -> Result<Json<Portal>, ServerError> {
      info!("GET /portal/{}", id.clone());
      let mut db_conn = db().await.map_err(|_| -> ServerError {ServerError::FailedToGetDbConnection})?;
      let get_query = sqlx::query_as::<_, Portal>("select * from portals where id = ?").bind(id);
      let portal = get_query.fetch_one(&mut db_conn).await.map_err(|_| -> ServerError {ServerError::FailedToFind})?;
      Ok(Json(portal))
    }

    pub async fn put_update_portal(Path(id): Path<String>, Json(portal): Json<Portal>) -> Result<Json<Portal>, ServerError> {
      info!("PUT /portal/{}: Updating portal config to {:?}", id, portal);
      let mut db_conn = db().await.map_err(|_| -> ServerError {ServerError::FailedToGetDbConnection})?;
      let put_query = sqlx::query("update portals set id=?, name=?, host=?, pos_x=?, pos_z=?, pos_y=? where id=?").bind(portal.id.to_string()).bind(portal.name.clone()).bind(portal.host.to_string()).bind(portal.pos_x).bind(portal.pos_z).bind(portal.pos_y).bind(id.to_string());
      let put_query_result = put_query.execute(&mut db_conn).await;
      debug!("{:?}", put_query_result);
      if put_query_result.is_ok() {
        return Ok(Json(portal));
      }

      return Err(ServerError::AlreadyExists);
    }

    pub async fn delete_portal(Path(id): Path<String>) -> Result<http::StatusCode, ServerError> {
      info!("DELETE /portal/{}", id);
      let mut db_conn = db().await.map_err(|_| -> ServerError {ServerError::FailedToGetDbConnection})?;
      let delete_query = sqlx::query("delete from portals where id=?").bind(&id);
      if delete_query.execute(&mut db_conn).await.is_ok() {
        return Ok(http::StatusCode::OK);
      }

      return Err(ServerError::FailedToFind);
    }
  }
}