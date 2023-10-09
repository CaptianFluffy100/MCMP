use axum::{ extract::Path, Form, Json, response::IntoResponse };
use serde::*;
use leptos::*;

use crate::{database, structs::server::Server};

pub async fn get_server_info(Path(uuid): Path<String>) -> impl IntoResponse {
    println!("Server UUID: {:?}", uuid);
    let server = Server { uuid: "asd54f5sa4".to_string(), ip: "0.0.0.0".to_string(), port: 25455, name: "Test Server".to_string()};
    let json = serde_json::json!({
        "status": "success",
        "message": format!("{:?}", server)
    });
    return Json(json);
}

pub async fn list_servers() -> impl IntoResponse {
    let servers = database::get_json_servers().await;
    
    let json = serde_json::json!(servers);
    return Json(json);
}