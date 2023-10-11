use crate::{error_template::{AppError, ErrorTemplate}, structs::{cob::GLaDOSError, portal::PortalVec}, app::{PopulateSideBar, get_portals}};
use leptos::{*, html::{Tr, Dialog}};
use leptos_meta::*;
use leptos_router::*;
use leptos::{error::Result, *};
use serde::{Deserialize, Serialize};
use stylers::style;

use crate::structs::server::ServerVec;

#[component]
pub fn PortalPageEdit() -> impl IntoView {
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
                {PortalPageEditDyn}
              </div> 
              {PopulateSideBar}
            </div>
        </div>
    }
}

#[component]
pub fn portal_page_edit_dyn() -> impl IntoView {
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
                                    <th>EDIT</th>
                                    <th>REMOVE</th>
                                  </tr>
                                </thead>
                                <tbody> 
                                {    
                                    let mut html: Vec<HtmlElement<Tr>> = vec![];
                                    let mut edit: Vec<HtmlElement<Dialog>> = vec![];
                                    match a {
                                        Ok(data) => {
                                            for portals in data.clone().portals {
                                                let open = portals.index.clone()+".showModal()";
                                                html.push(view! {<tr><th>{portals.index.clone()}</th><td>{portals.frameBlockId.clone()}</td><td>{portals.lightWithItemId.clone()}</td><td>{portals.color_b}</td><td>{portals.color_g}</td><td>{portals.color_r}</td><td><a class="btn btn-outline btn-warning" onclick={open}>EDIT</a></td><td><a class="btn btn-outline btn-error">REMOVE</a></td></tr>});
                                                edit.push(view! {<dialog id={portals.index.clone()} class="modal"><div class="modal-box"><h3 class="font-bold text-lg">Edit {portals.index.clone()}</h3><div class="py-4 grid grid-rows-3 grid-flow-col gap-4"><div><a>INDEX: </a><input type="text" placeholder="INDEX" class="input input-bordered w-full max-w-xs" value={portals.index.clone()} disabled /></div><div><a>Frame Block: </a><input type="text" placeholder="FRAME BLOCK" class="input input-bordered w-full max-w-xs" value={portals.frameBlockId.clone()} /></div><div><a>Light With Item: </a><input type="text" placeholder="Light With Item" class="input input-bordered w-full max-w-xs" value={portals.lightWithItemId.clone()} /></div><div><a>Color B: </a><input type="text" placeholder="Color B" class="input input-bordered w-full max-w-xs" value={portals.color_b.clone()} /></div><div><a>Color G: </a><input type="text" placeholder="Color G" class="input input-bordered w-full max-w-xs" value={portals.color_g.clone()} /></div><div><a>Color R: </a><input type="text" placeholder="Color R" class="input input-bordered w-full max-w-xs" value={portals.color_r.clone()} /></div></div><button class="btn btn-outline btn-success">SAVE</button></div><form method="dialog" class="modal-backdrop"><button>close</button></form></dialog>});
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