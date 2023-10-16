use crate::{error_template::{AppError, ErrorTemplate}, app::{PopulateSideBar, GladosMainBtn, get_portal_instances}, api::schema::Portal};
use leptos::{*, html::Tr};
use leptos_meta::*;
use leptos_router::*;
use leptos::{error::Result, *};
use serde::{Deserialize, Serialize};
use stylers::style;

#[component]
pub fn PortalInstancePage() -> impl IntoView {
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
                {GladosMainBtn}
                {PortalPageDyn}
              </div> 
              {PopulateSideBar}
            </div>
        </div>
    }
}

#[component]
pub fn portal_page_dyn() -> impl IntoView {
    let async_data: Resource<(), std::result::Result<Vec<Portal>, error::Error>> = create_local_resource(
        // the first is the "source signal"
        || (),
        // the second is the loader
        // it takes the source signal's value as its argument
        // and does some async work
        |_| async move { get_portal_instances().await },
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
                                    <th>ID</th>
                                    <th>Host</th>
                                    <th>Pos X</th>
                                    <th>Pos Y</th>
                                    <th>Pos Z</th>
                                  </tr>
                                </thead>
                                <tbody> 
                                {    
                                    let mut html: Vec<HtmlElement<Tr>> = vec![];
                                    match a {
                                        Ok(data) => {
                                            for portals in data.clone() {
                                                html.push(
                                                    view! {
                                                        <tr>
                                                            <th>{portals.name}</th>
                                                            <td>{portals.id.to_string()}</td>
                                                            <td>{portals.host.to_string()}</td>
                                                            <td>{portals.pos_x}</td>
                                                            <td>{portals.pos_y}</td>
                                                            <td>{portals.pos_z}</td>
                                                        </tr>
                                                    }
                                                );
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