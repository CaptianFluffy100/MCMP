use axum::{ extract::Path, Form, Json, response::IntoResponse };
use serde::*;
use leptos::*;

use crate::{database, structs::server::Server};

pub async fn edit_server(Path(uuid): Path<String>, Json(body): Json<Server>,) -> impl IntoResponse {
    println!("UUID: {:?}", uuid);
    println!("Edit Server: {:?}", body);
    let json = serde_json::json!({
        "status": "success",
        "message": "Updated Server"
    });
    return Json(json);
}