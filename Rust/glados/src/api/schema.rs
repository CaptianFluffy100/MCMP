use std::net::Ipv4Addr;
use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

cfg_if! { if #[cfg(feature = "ssr")] {
    use sqlx::Row;
  }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Server {
  pub id: Uuid,
  pub ip: Ipv4Addr,
  pub port: u16,
  pub name: String
}

#[cfg(feature = "ssr")]
impl sqlx::FromRow<'_, sqlx::sqlite::SqliteRow> for Server {
  fn from_row(row: &'_ sqlx::sqlite::SqliteRow) -> sqlx::Result<Self> {
    return if let Ok(id) = Uuid::parse_str(row.try_get("id")?) {
      Ok(Self {
        id,
        ip: Ipv4Addr::from(row.try_get::<u32, &str>("ip")?),
        port: row.try_get("port")?,
        name: row.try_get("name")?,
      })
    } else {
      Err(sqlx::Error::Protocol(String::from("Uuid is not parsable")))
    }
  }
}

impl Default for Server {
  fn default() -> Self {
    Server {
      id: Uuid::new_v4(),
      ip: Ipv4Addr::new(127, 0, 0, 1),
      port: 25565,
      name: "".to_string(),
    }
  }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServerStatus {
  online: bool,
  num_players: u64,
  max_players: u64,
  portals: u64
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServerStatusOverview {
  online: u64,
  servers: u64,
  players: u64,
  portals: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Player {
  name: String,
  id: Uuid,
  server: Uuid
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Portal {
  id: Uuid,
  name: String,
  host: Uuid
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub enum IgniteWith {
  Fluid(String),
  Item(String),
  #[default]
  Fire
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PortalConfig {
  name: String,
  id: Uuid,
  frame_block_id: String,
  color: ColorRGB,
  ignite_with: IgniteWith
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ColorRGB {
  red: u8,
  green: u8,
  blue: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaginatedResult<T> {
  total: u64,
  page: u64,
  per_page: u64,
  has_next: bool,
  has_previous: bool,
  results: Vec<T>
}
