#[cfg(feature = "ssr")]
use axum::{ extract::Path, Form, Json, response::IntoResponse };
use serde::*;
use leptos::*;

use crate::{database, structs::portal::Portal};

// pub async fn get_server_info(Path(uuid): Path<String>) -> impl IntoResponse {
//     println!("Server UUID: {:?}", uuid);
//     // let server = Portal { uuid: "asd54f5sa4".to_string(), ip: "0.0.0.0".to_string(), port: 25455, name: "Test Server".to_string()};
//     let json = serde_json::json!({
//         "status": "success",
//         // "message": format!("{:?}", server)
//     });
//     return Json(json);
// }
//
#[cfg(feature = "ssr")]
pub async fn list_portals() -> impl IntoResponse {
    let portals = database::get_json_portals().await;

    let json = serde_json::json!(portals);
    return Json(json);
}