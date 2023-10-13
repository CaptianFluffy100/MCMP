#[cfg(feature = "ssr")]
use axum::{ extract::Path, Form, Json, response::IntoResponse };
#[cfg(feature = "ssr")]
use serde::*;
#[cfg(feature = "ssr")]
use leptos::*;
use log::error;
use glados::api::handlers::db_setup;
use crate::state::ServerState;
// use serde::Deserialize;

pub mod database;
pub mod structs;
mod api;
mod error_template;
pub mod state;

#[cfg(feature = "ssr")]
#[derive(Deserialize, Debug)]
struct Server {
    uuid: String,
    name: String,
    ip: String,
    port: u16
}

pub fn context() -> ServerState {
    ServerState {}
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{routing::{post, get, put, delete}, Router};
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use glados::app::{*};
    use glados::fileserv::file_and_error_handler;

    simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

    let db = db_setup().await;
    if let Err(err) = db {
        error!("Failed to setup database. {}", err);
    }

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    // _ = ListServers::register();
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let state = ServerState {};

    let addr = leptos_options.site_addr.clone();
    let routes = generate_route_list(App);

    // build our application with a route
    let app = Router::new()
        // .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .route("/api/server", get(api::handlers::get_list_servers).post(api::handlers::post_json_register_server))
        .route("/api/server/:id", get(api::handlers::get_server_by_id).put(api::handlers::put_json_update_server).delete(api::handlers::delete_unregister_server))
        // .route("/api/servers", post(api::server::post::add_server))
        // .route("/api/servers/:uuid", get(api::server::get::get_server_info))
        // .route("/api/servers/:uuid", post(api::server::post::edit_server))
        // .route("/api/portals", get(api::portal::get::list_portals))
        // .route("/api/portals", post(api::server::post::add_server))
        .leptos_routes(&leptos_options, routes, App)
        .fallback(file_and_error_handler)
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log::info!("listening on http://{}", &addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
