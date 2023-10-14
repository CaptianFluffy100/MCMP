use std::net::Ipv4Addr;
use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

cfg_if! { if #[cfg(feature = "ssr")] {
    use sqlx::Row;
    use crate::api::error::ServerError;
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
  num_players: i64,
  max_players: i64,
  portals: i64
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TimestampedServerStatus {
  timestamp: time::OffsetDateTime,
  status: ServerStatus
}

impl TimestampedServerStatus {
  pub fn now(status: ServerStatus) -> Self {
    Self {
      timestamp: OffsetDateTime::now_utc(),
      status
    }
  }
}

#[cfg(feature = "ssr")]
impl sqlx::FromRow<'_, sqlx::sqlite::SqliteRow> for TimestampedServerStatus {
  fn from_row(row: &'_ sqlx::sqlite::SqliteRow) -> sqlx::Result<Self> {
    let timestamp = OffsetDateTime::parse(row.try_get("timestamp")?, &time::format_description::well_known::Iso8601::DEFAULT).unwrap_or(OffsetDateTime::now_utc());
    let status = ServerStatus {
      online: row.try_get("online")?,
      num_players: row.try_get("num_players")?,
      max_players: row.try_get("max_players")?,
      portals: row.try_get("portals")?,
    };

    Ok(TimestampedServerStatus{
      timestamp,
      status,
    })
  }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServerOverview {
  online: i64,
  servers: i64,
  players: i64,
  portals: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TimestampedServerOverview {
  timestamp: time::OffsetDateTime,
  overview: ServerOverview
}

impl TimestampedServerOverview {
  pub fn now(overview: ServerOverview) -> Self {
    Self {
      timestamp: OffsetDateTime::now_utc(),
      overview
    }
  }
}

#[cfg(feature = "ssr")]
impl sqlx::FromRow<'_, sqlx::sqlite::SqliteRow> for TimestampedServerOverview {
  fn from_row(row: &'_ sqlx::sqlite::SqliteRow) -> sqlx::Result<Self> {
    let timestamp = OffsetDateTime::parse(row.try_get("timestamp")?, &time::format_description::well_known::Iso8601::DEFAULT).unwrap_or(OffsetDateTime::now_utc());
    let overview = ServerOverview {
      online: row.try_get("online")?,
      players: row.try_get("players")?,
      servers: row.try_get("servers")?,
      portals: row.try_get("portals")?,
    };

    Ok(TimestampedServerOverview{
      timestamp,
      overview,
    })
  }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Player {
  pub name: String,
  pub id: Uuid,
  pub server: Uuid
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Portal {
  pub id: Uuid,
  pub name: String,
  pub host: Uuid,
  pub pos_x: i64,
  pub pos_z: i64,
  pub pos_y: i64,
}

#[cfg(feature = "ssr")]
impl sqlx::FromRow<'_, sqlx::sqlite::SqliteRow> for Portal {
  fn from_row(row: &'_ sqlx::sqlite::SqliteRow) -> sqlx::Result<Self> {
    let id = Uuid::parse_str(row.try_get("id")?).unwrap_or_default();
    let name = row.try_get("name")?;
    let host = Uuid::parse_str(row.try_get("host")?).unwrap_or_default();
    let pos_x = row.try_get::<i64, &str>("pos_x")?;
    let pos_z = row.try_get::<i64, &str>("pos_z")?;
    let pos_y = row.try_get::<i64, &str>("pos_y")?;

    return Ok(Portal {
      id,
      name,
      host,
      pos_x,
      pos_z,
      pos_y
    });
  }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(tag = "ignite_with", content = "ignite_with_id")]
#[serde(rename_all = "lowercase")]
pub enum IgniteWith {
  Fluid(String),
  Item(String),
  #[default]
  Fire
}

impl IgniteWith {
  pub fn ignite_with_id(ignite_with: &str, ignite_with_id: &str) -> Self {
    match ignite_with {
        "item" => Self::Item(ignite_with_id.to_string()),
        "fluid" => Self::Fluid(ignite_with_id.to_string()),
        _ => Self::Fire,
    }
  }

  pub fn deconstruct(&self) -> (String, String) {
    match self {
        Self::Fire => ("fire".into(), "".into()),
        Self::Fluid(id) => ("fluid".into(), id.clone()),
        Self::Item(id) => ("item".into(), id.clone()),
    }
  }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PortalConfig {
  pub name: String,
  pub id: Uuid,
  pub frame_block_id: String,
  pub color: ColorRGB,
  #[serde(flatten)]
  pub ignite_with: IgniteWith
}

#[cfg(feature = "ssr")]
impl sqlx::FromRow<'_, sqlx::sqlite::SqliteRow> for PortalConfig {
  fn from_row(row: &'_ sqlx::sqlite::SqliteRow) -> sqlx::Result<Self> {
    let id = Uuid::parse_str(row.try_get("id")?).unwrap_or_default();
    let frame_block_id = row.try_get("frame_block_id")?;
    let name = row.try_get("name")?;
    let color_bin: i32 = row.try_get("color")?;
    let color = ColorRGB {
      red: (color_bin >> 16 & 0xff) as u8,
      green: (color_bin >> 8 & 0xff) as u8,
      blue: (color_bin & 0xff) as u8,
    };
    let ignite_with_str = row.try_get("ignite_with")?;
    if let Ok(ignite_with_id) = row.try_get("ignite_with_id") {
      let ignite_with = IgniteWith::ignite_with_id(ignite_with_str, ignite_with_id);
      return Ok(PortalConfig {
        name,
        frame_block_id,
        color,
        ignite_with,
        id
      });
    }

    return Ok(PortalConfig {
      name,
      frame_block_id,
      color,
      ignite_with: IgniteWith::Fire,
      id
    });
  }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ColorRGB {
  pub red: u8,
  pub green: u8,
  pub blue: u8,
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
