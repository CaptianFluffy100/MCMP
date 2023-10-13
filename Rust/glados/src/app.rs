use std::collections::HashMap;

use crate::{error_template::{AppError, ErrorTemplate}, structs::{cob::GLaDOSError, portal::PortalVec, server::Server}, pages::{server_list::page::ServerPage, portal_list::page::PortalPage, server_edit::page::ServerPageEdit, portal_edit::page::PortalPageEdit, home::page::HomePage}};
use http::header::CONTENT_TYPE;
use leptos::{*, html::Tr};
use leptos_meta::*;
use leptos_router::*;
use leptos::{error::Result, *};
use reqwasm::http::RequestMode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use stylers::style;
use log::error;

use crate::api::schema::Server;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <html class="h-full" style="height: 100%;">
            // injects a stylesheet into the document <head>
            // id=leptos means cargo-leptos will hot-reload this stylesheet
            <Stylesheet id="leptos" href="/pkg/glados.css"/>
            // <Stylesheet id="tailwind" href="https://unpkg.com/tailwindcss@^1.0/dist/tailwind.min.css"/>
            // <Stylesheet id="daisyui" href="https://cdn.jsdelivr.net/npm/daisyui@3.9.2/dist/full.css"/>

            <Script id="htmx" src="https://unpkg.com/htmx.org@1.9.6"/>

            // sets the document title
            <Title text="Welcome to GLaDOS"/>

            // content for this welcome page
            <body class="h-full" style="height: 100%;">
                <Router fallback=|| {
                    let mut outside_errors = Errors::default();
                    outside_errors.insert_with_default_key(AppError::NotFound);
                    view! {
                        <ErrorTemplate outside_errors/>
                    }
                    .into_view()
                }>  
                    <main onload="get_server_list" class="flex max-h-screen flex-col items-center justify-between" style="height: 100%;">
                        <Routes>
                            <Route path="" view=HomePage/>
                            <Route path="/servers" view=ServerPage/>
                            <Route path="/portals" view=PortalPage/>
                            <Route path="/server/list" view=ServerPageEdit/>
                            <Route path="/portal/list" view=PortalPageEdit/>
                        </Routes>
                    </main>
                </Router>
            </body>
        </html>    
    }
}

// pub async fn get_server(uuid: String) -> Result<Server> {
//     let res = reqwasm::http::Request::get(&format!(
//         "/api/servers/{}", uuid,
//     ))
//     .send()
//     .await?
//     .json::<Server>()
//     .await?;
//     Ok(res)
//     // Err(GLaDOSError::ERROR.into())
// }

pub async fn get_servers() -> Result<ServerVec> {
    log::debug!("Get Servers");
    let res = reqwasm::http::Request::get(&format!(
        "/api/server",
    ))
    .send()
    .await?
    .json::<Vec<Server>>()
    .await?;
    Ok(res)
    // Err(GLaDOSError::ERROR.into())
}

pub async fn get_server(uuid: String) -> Result<Server> {
    let res = reqwasm::http::Request::get(&format!(
        "/api/servers/{}", uuid,
    ))
    .send()
    .await?
    .json::<Server>()
    .await?;
    Ok(res)
    // Err(GLaDOSError::ERROR.into())
}

pub async fn put_server(uuid: String, ip: String, port: u16, name: String) -> Result<Server> {
    log::debug!("Content PUT: {}", uuid);
    // let content = "{\"uuid\":"+uuid.to_owned()+",\"name\":"+name+",\"ip\":"+ip+",\"port\":"+port+"}";
    let content = json!({
        "uuid": uuid,
        "name": name,
        "ip": ip,
        "port": port,
    });
    log::debug!("Content PUT: {}", content);
    let body = serde_json::to_string(&content).expect("Failed to serialize JSON");
    let res = reqwasm::http::Request::put(&format!(
        "/api/servers/{}", &uuid,
    ))
    .header("Content-Type", "application/json")
    .body(body)
    .send()
    .await?
    .json::<Server>()
    .await?;
    Ok(res)
    // Err(GLaDOSError::ERROR.into())
}

pub async fn post_server(uuid: String, ip: String, port: u16, name: String) -> Result<Server> {
    // let content = "{\"uuid\":"+uuid.to_owned()+",\"name\":"+name+",\"ip\":"+ip+",\"port\":"+port+"}";
    let content = json!({
        "uuid": uuid,
        "name": name,
        "ip": ip,
        "port": port,
    });
    let body = serde_json::to_string(&content).expect("Failed to serialize JSON");
    let res = reqwasm::http::Request::post(&format!(
        "/api/servers",
    ))
    .header("Content-Type", "application/json")
    .body(body)
    .send()
    .await?
    .json::<Server>()
    .await?;
    Ok(res)
    // Err(GLaDOSError::ERROR.into())
}

pub async fn delete_server(uuid: String) -> Result<Server> {
    // let content = "{\"uuid\":"+uuid.to_owned()+",\"name\":"+name+",\"ip\":"+ip+",\"port\":"+port+"}";
    // let content = json!({
    //     "uuid": uuid,
    //     "name": name,
    //     "ip": ip,
    //     "port": port,
    // });
    // let body = serde_json::to_string(&content).expect("Failed to serialize JSON");
    let res = reqwasm::http::Request::delete(&format!(
        "/api/servers/{}", uuid,
    ))
    // .header("Content-Type", "application/json")
    // .body(body)
    .send()
    .await?
    .json::<Server>()
    .await?;
    Ok(res)
    // Err(GLaDOSError::ERROR.into())
}

// TODO Delete server based on UUID

pub async fn get_portals() -> Result<PortalVec> {
    let res = reqwasm::http::Request::get(&format!(
        "/api/portals",
    ))
    .send()
    .await?
    .json::<PortalVec>()
    .await?;
    Ok(res)
    // Err(GLaDOSError::ERROR.into())
}

#[component]
pub fn PopulateSideBar() -> impl IntoView {

    view! {
        <div class="drawer-side h-full" style="height: 100%;">
          <label for="my-drawer" aria-label="close sidebar" class="drawer-overlay"></label>
          <ul class="menu p-4 w-80 h-full bg-base-200 text-base-content" style="height: 100%;">
            <li><a href="/">Home Page</a></li>
            // <li class="w-full text-center underline align-center content-center font-black pt-6">List</li>
            <li>
                <details close>
                    <summary class="w-full underline font-black">List</summary>
                    <ul>
                        <li><a href="/servers">Server List</a></li>
                        <li><a href="/portals">Portal List</a></li>
                    </ul>
                </details>
            </li>
            // <li class="w-full text-center underline align-center content-center font-black pt-6">Add/Remove/Edit</li>
            // <li><a href="/server/list">Servers</a></li>
            // <li><a href="/portal/list">Portals</a></li>
            <li>
                <details close>
                    <summary class="w-full underline font-black">Add/Remove/Edit</summary>
                    <ul>
                        <li><a href="/server/list">Servers</a></li>
                        <li><a href="/portal/list">Portals</a></li>
                    </ul>
                </details>
            </li>
          </ul>
        </div>
    }
}

#[component]
pub fn GladosMainBtn() -> impl IntoView {

    view! {
        <label for="my-drawer" class="btn btn-ghost drawer-button">
            <label>
              <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="inline-block w-6 h-6 stroke-current">
                <path stroke="blue" stroke-linecap="round" stroke-linejoin="round" stroke-width="1" d="M 12 8 Q 4 4 8 12 M 12 8 Q 20 4 16 12 M 16 12 Q 20 20 12 16 M 12 16 Q 4 20 8 12 M 7 7 Q 12 16 17 7 M 17 17 Q 12 8 7 17 M 17 7 Q 8 12 17 17 M 7 7 Q 16 12 7 17"></path>
                <path stroke="red" stroke-linecap="round" stroke-linejoin="round" stroke-width="1" d="M 11 11 L 13 11 L 13 13 L 11 13 L 11 11 L 13 13"></path>
                <path stroke="white" stroke-linecap="round" stroke-linejoin="round" stroke-width="1" d="M 2 12 C 2 0 22 0 22 12 M 2 12 C 2 25 22 25 22 12"></path>
              </svg>
            </label>
            GLaDOS
        </label>
    }
}