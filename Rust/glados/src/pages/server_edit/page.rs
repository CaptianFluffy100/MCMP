use crate::{error_template::{AppError, ErrorTemplate}, structs::{cob::GLaDOSError, portal::PortalVec}, app::{PopulateSideBar, get_servers}};
use leptos::{*, html::{Tr, Dialog}};
use leptos_meta::*;
use leptos_router::*;
use leptos::{error::Result, *};
use serde::{Deserialize, Serialize};
use stylers::style;

use crate::structs::server::ServerVec;

#[component]
pub fn ServerPageEdit() -> impl IntoView {
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
                <a class="btn btn-outline float-right btn-success">ADD</a>
                {ServerPageEditDyn}
              </div> 
              {PopulateSideBar}
            </div>
        </div>
    }
}

#[component]
pub fn server_page_edit_dyn() -> impl IntoView {
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
                                    <th>EDIT</th>
                                    <th>REMOVE</th>
                                  </tr>
                                </thead>
                                <tbody> 
                                {    
                                    let mut html: Vec<HtmlElement<Tr>> = vec![];
                                    let mut edit: Vec<HtmlElement<Dialog>> = vec![];
                                    // if html.len() == 0 {
                                    //     html = html + &format!("{:?}", a);
                                    // }
                                    // format!("{:?}", a);
                                    match a {
                                        Ok(data) => {
                                            for server in data.clone().servers {
                                                let open = server.uuid.clone()+".showModal()";
                                                html.push(view! {<tr><th>{server.name.clone()}</th><td>{server.uuid.clone()}</td><td>{server.ip.clone()}</td><td>{server.port}</td><td><a class="btn btn-outline btn-warning" onclick={open}>EDIT</a></td><td><a class="btn btn-outline btn-error">REMOVE</a></td></tr>});
                                                edit.push(view! {<dialog id={server.uuid.clone()} class="modal"><div class="modal-box"><h3 class="font-bold text-lg">Edit {server.name.clone()}</h3><div class="py-4 grid grid-rows-2 grid-flow-col gap-4"><div><a>UUID: </a><input type="text" placeholder="UUID" class="input input-bordered w-full max-w-xs" value={server.uuid.clone()} disabled /></div><div><a>NAME: </a><input type="text" placeholder="NAME" class="input input-bordered w-full max-w-xs" value={server.name.clone()} /></div><div><a>IP: </a><input type="text" placeholder="IP" class="input input-bordered w-full max-w-xs" value={server.ip.clone()} /></div><div><a>PORT: </a><input type="text" placeholder="PORT" class="input input-bordered w-full max-w-xs" value={server.port.clone()} /></div></div><button class="btn btn-outline btn-success">SAVE</button></div><form method="dialog" class="modal-backdrop"><button>close</button></form></dialog>});
                                            }
                                            (html, edit)
                                        },
                                        Err(e) => {
                                            html.push(view! {<tr>{format!("{:?}", e)}</tr>});
                                            // TODO
                                            (html, edit)
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