use axum::{ extract::Path, Form, Json, response::IntoResponse };
use serde::*;
use leptos::*;

use crate::{database, structs::server::Server};

pub async fn add_server(Json(body): Json<Server>,) -> impl IntoResponse {
    println!("New Server: {:?}", body);
    let json = serde_json::json!({
        "status": "success",
        "message": "Added new Server"
    });
    return Json(json);
}

pub async fn edit_server(Json(body): Json<Server>,) -> impl IntoResponse {
    println!("Edit Server: {:?}", body);
    let json = serde_json::json!({
        "status": "success",
        "message": "Added new Server"
    });
    return Json(json);
}