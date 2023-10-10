use crate::{error_template::{AppError, ErrorTemplate}, structs::{cob::GLaDOSError, portal::PortalVec}, app::{PopulateSideBar, get_servers}};
use leptos::{*, html::Tr};
use leptos_meta::*;
use leptos_router::*;
use leptos::{error::Result, *};
use serde::{Deserialize, Serialize};
use stylers::style;

use crate::structs::server::ServerVec;

#[component]
pub fn ServerPage() -> impl IntoView {
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
              {PopulateSideBar}
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