use crate::{error_template::{AppError, ErrorTemplate}, structs::{cob::GLaDOSError, portal::PortalVec}};
use leptos::{*, html::Tr};
use leptos_meta::*;
use leptos_router::*;
use leptos::{error::Result, *};
use serde::{Deserialize, Serialize};
use stylers::style;

use crate::structs::server::ServerVec;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <html class="h-full" style="height: 100%;">
            // injects a stylesheet into the document <head>
            // id=leptos means cargo-leptos will hot-reload this stylesheet
            // <Stylesheet id="leptos" href="/pkg/glados.css"/>
            // <Stylesheet id="tailwind" href="https://unpkg.com/tailwindcss@^1.0/dist/tailwind.min.css"/>
            <Stylesheet id="daisyui" href="https://cdn.jsdelivr.net/npm/daisyui@3.9.2/dist/full.css"/>

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
                        </Routes>
                    </main>
                </Router>
            </body>
        </html>    
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    view! {
        <div class="navbar bg-base-100 h-full" style="height: 100%;">
            <div class="drawer h-full" style="height: 100%;">
              <input id="my-drawer" type="checkbox" class="drawer-toggle" />
              <div class="drawer-content" style="height: 100%;">
                // <div inner-html={page_data}/>
                <label for="my-drawer" class="btn btn-ghost drawer-button">GLaDOS</label>
                // {ServerPageDyn}
              </div> 
              <div class="drawer-side h-full" style="height: 100%;">
                <label for="my-drawer" aria-label="close sidebar" class="drawer-overlay"></label>
                <ul class="menu p-4 w-80 h-full bg-base-200 text-base-content" style="height: 100%;">
                  <li><a href="/">Home Page</a></li>
                  <li><a href="/servers">Server List</a></li>
                  <li><a href="/portals">Portal List</a></li>
                </ul>
              </div>
            </div>
        </div>
    }
}

async fn get_servers() -> Result<ServerVec> {
    let res = reqwasm::http::Request::get(&format!(
        "/api/servers",
    ))
    .send()
    .await?
    .json::<ServerVec>()
    .await?;
    Ok(res)
    // Err(GLaDOSError::ERROR.into())
}

#[component]
fn ServerPage() -> impl IntoView {
    // Creates a reactive value to update the button
    // let (count, set_count) = create_signal(0);
    // let on_click = move |_| set_count.update(|count| *count += 1);
    // let get_server_list = move |_| set_count.update(|count| *count += 1);
    // let (servers, set_servers) = create_signal();

    view! {
        <div class="navbar bg-base-100 h-full" style="height: 100%;">
            <div class="drawer h-full" style="height: 100%;">
              <input id="my-drawer" type="checkbox" class="drawer-toggle" />
              <div class="drawer-content" style="height: 100%;">
                // <div inner-html={page_data}/>
                <label for="my-drawer" class="btn btn-ghost drawer-button">GLaDOS</label>
                {ServerPageDyn}
              </div> 
              <div class="drawer-side h-full" style="height: 100%;">
                <label for="my-drawer" aria-label="close sidebar" class="drawer-overlay"></label>
                <ul class="menu p-4 w-80 h-full bg-base-200 text-base-content" style="height: 100%;">
                  <li><a href="/">Home Page</a></li>
                  <li><a href="/servers">Server List</a></li>
                  <li><a href="/portals">Portal List</a></li>
                </ul>
              </div>
            </div>
        </div>
    }
}

#[component]
pub fn server_page_dyn() -> impl IntoView {
    let async_data: Resource<(), std::result::Result<ServerVec, error::Error>> = create_local_resource(
        // the first is the "source signal"
        || (),
        // the second is the loader
        // it takes the source signal's value as its argument
        // and does some async work
        |_| async move { get_servers().await },
    );

    view! {
        <Suspense
            fallback=move || view! { <p class="place-content-center"><span class="loading loading-infinity loading-lg"></span></p> }>
            {move || {
                async_data.get()
                    .map(|a| view! { 
                        // Display Table
                        <div class="overflow-x-auto">
                            <table class="table table-zebra">
                                <thead>
                                  <tr>
                                    <th>Name</th>
                                    <th>UUID</th>
                                    <th>IP</th>
                                    <th>PORT</th>
                                  </tr>
                                </thead>
                                <tbody> 
                                {    
                                    let mut html: Vec<HtmlElement<Tr>> = vec![];
                                    // if html.len() == 0 {
                                    //     html = html + &format!("{:?}", a);
                                    // }
                                    // format!("{:?}", a);
                                    match a {
                                        Ok(data) => {
                                            for server in data.clone().servers {
                                                html.push(view! {<tr><th>{server.name}</th><td>{server.uuid}</td><td>{server.ip}</td><td>{server.port}</td></tr>});
                                            }
                                            html
                                        },
                                        Err(e) => {
                                            html.push(view! {<tr>{format!("{:?}", e)}</tr>});
                                            // TODO
                                            html
                                        }
                                    }
                                }
                               </tbody>
                           </table>
                       </div>
                })
            }}
        </Suspense>
    }
}

async fn get_portals() -> Result<PortalVec> {
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
fn PortalPage() -> impl IntoView {
    // Creates a reactive value to update the button
    // let (count, set_count) = create_signal(0);
    // let on_click = move |_| set_count.update(|count| *count += 1);
    // let get_server_list = move |_| set_count.update(|count| *count += 1);
    // let (servers, set_servers) = create_signal();

    view! {
        <div class="navbar bg-base-100 h-full" style="height: 100%;">
            <div class="drawer h-full" style="height: 100%;">
              <input id="my-drawer" type="checkbox" class="drawer-toggle" />
              <div class="drawer-content" style="height: 100%;">
                // <div inner-html={page_data}/>
                <label for="my-drawer" class="btn btn-ghost drawer-button">GLaDOS</label>
                {PortalPageDyn}
              </div> 
              <div class="drawer-side h-full" style="height: 100%;">
                <label for="my-drawer" aria-label="close sidebar" class="drawer-overlay"></label>
                <ul class="menu p-4 w-80 h-full bg-base-200 text-base-content" style="height: 100%;">
                  <li><a href="/">Home Page</a></li>
                  <li><a href="/servers">Server List</a></li>
                  <li><a href="/portals">Portal List</a></li>
                </ul>
              </div>
            </div>
        </div>
    }
}

#[component]
pub fn portal_page_dyn() -> impl IntoView {
    let async_data: Resource<(), std::result::Result<PortalVec, error::Error>> = create_local_resource(
        // the first is the "source signal"
        || (),
        // the second is the loader
        // it takes the source signal's value as its argument
        // and does some async work
        |_| async move { get_portals().await },
    );

    view! {
        <Suspense
            fallback=move || view! { <p class="place-content-center"><span class="loading loading-infinity loading-lg"></span></p> }>
            {move || {
                async_data.get()
                    .map(|a| view! { 
                        // Display Table
                        <div class="overflow-x-auto">
                            <table class="table table-zebra">
                                <thead>
                                  <tr>
                                    <th>Index</th>
                                    <th>Frame Block</th>
                                    <th>Ligth With Item</th>
                                    <th>Color B</th>
                                    <th>Color G</th>
                                    <th>Color R</th>
                                  </tr>
                                </thead>
                                <tbody> 
                                {    
                                    let mut html: Vec<HtmlElement<Tr>> = vec![];
                                    match a {
                                        Ok(data) => {
                                            for portals in data.clone().portals {
                                                html.push(view! {<tr><th>{portals.index}</th><td>{portals.frameBlockId}</td><td>{portals.lightWithItemId}</td><td>{portals.color_b}</td><td>{portals.color_g}</td><td>{portals.color_r}</td></tr>});
                                            }
                                            html
                                        },
                                        Err(e) => {
                                            html.push(view! {<tr>{format!("{:?}", e)}</tr>});
                                            // TODO
                                            html
                                        }
                                    }
                                }
                               </tbody>
                           </table>
                       </div>
                })
            }}
        </Suspense>
    }
}