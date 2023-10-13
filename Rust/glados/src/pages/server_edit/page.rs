use crate::{error_template::{AppError, ErrorTemplate}, structs::{cob::GLaDOSError, portal::PortalVec, server::Server}, app::{PopulateSideBar, get_servers, GladosMainBtn, put_server}};
use leptos::{*, html::{Tr, Dialog, Input}, ev::SubmitEvent};
use leptos_meta::*;
use leptos_router::*;
use leptos::{error::Result, *};
use serde::{Deserialize, Serialize};
use stylers::style;
use wasm_bindgen_futures::spawn_local;

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
                {GladosMainBtn}
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
    // let (read_put_uuid, write_put_uuid) = create_signal(0);
    // let input_element: NodeRef<Input> = create_node_ref();

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
                                                let edit_props = ServerPageEditFormsDynProps{name: server.name.clone(), uuid: server.uuid.clone(), ip: server.ip.clone(), port: server.port.clone()};
                                                edit.push(view! {<dialog id={server.uuid.clone()} class="modal">{ServerPageEditFormsDyn(edit_props)}</dialog>});
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

#[component]
pub fn server_page_edit_forms_dyn(name: String, uuid: String, ip: String, port: u16) -> impl IntoView {
    let (read_put_uuid, write_put_uuid) = create_signal(uuid.clone());
    let (read_put_name, write_put_name) = create_signal(name.clone());
    let (read_put_ip, write_put_ip) = create_signal(ip.clone());
    let (read_put_port, write_put_port) = create_signal(port);

    let input_element_uuid: NodeRef<Input> = create_node_ref();
    let input_element_name: NodeRef<Input> = create_node_ref();
    let input_element_ip: NodeRef<Input> = create_node_ref();
    let input_element_port: NodeRef<Input> = create_node_ref();

    let async_data: Resource<(), std::result::Result<ServerVec, error::Error>> = create_local_resource(
        // the first is the "source signal"
        || (),
        // the second is the loader
        // it takes the source signal's value as its argument
        // and does some async work
        |_| async move { get_servers().await },
    );

    let on_submit = move |ev: SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        // here, we'll extract the value from the input
        let value_uuid = input_element_uuid()
            .expect("<input> to exist")
            .value();
        let value_name = input_element_name()
            .expect("<input> to exist")
            .value();
        let value_ip = input_element_ip()
            .expect("<input> to exist")
            .value();
        let value_port: u16 = input_element_port()
            .expect("<input> to exist")
            .value()
            .parse()
            .unwrap();
        // write_put_uuid(value);
        log::debug!("Value: {}", value_uuid.clone());
        log::debug!("Value: {}", value_name.clone());
        log::debug!("Value: {}", value_ip.clone());
        log::debug!("Value: {}", value_port.clone());

        let _uuid = value_uuid.clone();
        let _ip = value_ip.clone();
        let _port = value_port.clone();
        let _name = value_name.clone();
        
        let stable = create_local_resource( move || (value_uuid.clone(),value_ip.clone(),value_port.clone(),value_name.clone()), 
        move |(_uuid, _ip, _port, _name)| async move { put_server(_uuid.clone(), _ip.clone(), _port.clone(), _name.clone()).await });
    };

    

    view! {
        <form on:submit=on_submit>
            <div class="modal-box">
                <h3 class="font-bold text-lg">Edit {name.clone()}</h3>
                <div class="py-4 grid grid-rows-2 grid-flow-col gap-4">
                    <div>
                        <a>UUID: </a>
                        <input type="text" placeholder="UUID" class="input input-bordered w-full max-w-xs" value=read_put_uuid node_ref=input_element_uuid />
                    </div>
                    <div>
                        <a>NAME: </a>
                        <input type="text" placeholder="NAME" class="input input-bordered w-full max-w-xs" value=read_put_name node_ref=input_element_name />
                    </div>
                    <div>
                        <a>IP: </a>
                        <input type="text" placeholder="IP" class="input input-bordered w-full max-w-xs" value=read_put_ip node_ref=input_element_ip />
                    </div>
                    <div>
                        <a>PORT: </a>
                        <input type="text" placeholder="PORT" class="input input-bordered w-full max-w-xs" value=read_put_port node_ref=input_element_port />
                    </div>
                </div>

                <input class="btn btn-outline btn-success" type="submit" on:submit=on_submit value="SAVE"/>
                

            </div>
            <form method="dialog" class="modal-backdrop"><button>close</button></form>
        </form>
    }

}