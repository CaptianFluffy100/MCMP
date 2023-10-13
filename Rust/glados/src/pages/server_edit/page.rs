use crate::{error_template::{AppError, ErrorTemplate}, structs::{cob::GLaDOSError, portal::PortalVec}, app::{PopulateSideBar, get_servers, GladosMainBtn, put_server, post_server, delete_server}, api::schema::Server};
use leptos::{*, html::{Tr, Dialog, Input}, ev::SubmitEvent};
use leptos_meta::*;
use leptos_router::*;
use leptos::{error::Result, *};
use serde::{Deserialize, Serialize};
use stylers::style;

use crate::structs::server::ServerVec;

#[component]
pub fn ServerPageEdit() -> impl IntoView {
    let (read_put_uuid, write_put_uuid) = create_signal("".to_string());
    let (read_put_name, write_put_name) = create_signal("".to_string());
    let (read_put_ip, write_put_ip) = create_signal("0.0.0.0".to_string());
    let (read_put_port, write_put_port) = create_signal(25565);

    let input_element_uuid: NodeRef<Input> = create_node_ref();
    let input_element_name: NodeRef<Input> = create_node_ref();
    let input_element_ip: NodeRef<Input> = create_node_ref();
    let input_element_port: NodeRef<Input> = create_node_ref();

    let async_data: Resource<(), std::result::Result<Vec<Server>, error::Error>> = create_local_resource(
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
        // log::debug!("Value: {}", value_uuid.clone());
        // log::debug!("Value: {}", value_name.clone());
        // log::debug!("Value: {}", value_ip.clone());
        // log::debug!("Value: {}", value_port.clone());

        // let _uuid = value_uuid.clone();
        // let _ip = value_ip.clone();
        // let _port = value_port.clone();
        // let _name = value_name.clone();
        
        let stable = create_local_resource( move || (value_uuid.clone(),value_ip.clone(),value_port.clone(),value_name.clone()), 
        move |(_uuid, _ip, _port, _name)| async move { post_server(_uuid.clone(), _ip.clone(), _port.clone(), _name.clone()).await });
    };

    let open = "add.showModal()";
    view! {
        <div class="navbar bg-base-100 h-full" style="height: 100%;">
            <div class="drawer h-full" style="height: 100%;">
              <input id="my-drawer" type="checkbox" class="drawer-toggle" />
              <div class="drawer-content" style="height: 100%;">
                // <div inner-html={page_data}/>
                {GladosMainBtn}
                <a class="btn btn-outline float-right btn-success" onclick={open}>ADD</a>
                <dialog id="add" class="modal">
                    <form on:submit=on_submit>
                        <div class="modal-box">
                            <h3 class="font-bold text-lg">Add New Server</h3>
                            <div class="py-4 grid grid-rows-2 grid-flow-col gap-4">
                                <div>
                                    <a>UUID: </a>
                                    <input type="text" placeholder="UUID" class="input input-bordered w-full max-w-xs" value=read_put_uuid node_ref=input_element_uuid/>
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
                            <div class="grid grid-rows-1 gap-4">
                                <input class="btn btn-outline btn-success" type="submit" on:submit=on_submit value="SAVE"/>
                            </div>
                        </div>
                    </form>
                </dialog>
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

    let async_data: Resource<(), std::result::Result<Vec<Server>, error::Error>> = create_local_resource(
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
                                    let mut remove: Vec<HtmlElement<Dialog>> = vec![];
                                    // if html.len() == 0 {
                                    //     html = html + &format!("{:?}", a);
                                    // }
                                    // format!("{:?}", a);
                                    match a {
                                        Ok(data) => {
                                            for server in data.clone() {
                                                let open_edit = "edit".to_string()+&server.id.clone().to_string().replace("-", "")+".showModal()";
                                                let open_remove = "remove".to_string()+&server.id.clone().to_string().replace("-", "")+".showModal()";
                                                html.push(view! {<tr><th>{server.name.clone()}</th><td>{server.id.clone().to_string()}</td><td>{server.ip.clone()}</td><td>{server.port}</td><td><a class="btn btn-outline btn-warning" onclick={open_edit}>EDIT</a></td><td><a class="btn btn-outline btn-error" onclick={open_remove}>REMOVE</a></td></tr>});
                                                let edit_props = ServerPageEditFormsDynProps{name: server.name.clone(), uuid: server.id.clone().to_string(), ip: server.ip.clone().to_string(), port: server.port.clone()};
                                                edit.push(view! {<dialog id={"edit".to_string()+&server.id.clone().to_string().replace("-", "")} class="modal">{ServerPageEditFormsDyn(edit_props)}</dialog>});
                                                let remove_props = ServerPageRemoveFormsDynProps{uuid: server.id.clone().to_string(), name: server.name.clone().to_string()};
                                                remove.push(view! {<dialog id={"remove".to_string()+&server.id.clone().to_string().replace("-", "")} class="modal">{ServerPageRemoveFormsDyn(remove_props)}</dialog>});
                                            }
                                            (html, edit, remove)
                                        },
                                        Err(e) => {
                                            html.push(view! {<tr>{format!("{:?}", e)}</tr>});
                                            // TODO
                                            (html, edit, remove)
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
        // log::debug!("Value: {}", value_uuid.clone());
        // log::debug!("Value: {}", value_name.clone());
        // log::debug!("Value: {}", value_ip.clone());
        // log::debug!("Value: {}", value_port.clone());

        // let _uuid = value_uuid.clone();
        // let _ip = value_ip.clone();
        // let _port = value_port.clone();
        // let _name = value_name.clone();
        
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
                        <input type="text" placeholder="UUID" class="input input-bordered w-full max-w-xs" value=read_put_uuid node_ref=input_element_uuid disabled/>
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
                <div class="grid grid-rows-1 gap-4">
                    <input class="btn btn-outline btn-success" type="submit" on:submit=on_submit value="SAVE"/>
                </div>
            </div>
        </form>
    }

}

#[component]
pub fn server_page_remove_forms_dyn(name: String, uuid: String) -> impl IntoView {
    let (read_put_uuid, write_put_uuid) = create_signal("".to_string());

    let input_element_uuid: NodeRef<Input> = create_node_ref();

    let uuid_org = uuid.clone();

    let on_submit = move |ev: SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        // here, we'll extract the value from the input
        let value_uuid = input_element_uuid()
            .expect("<input> to exist")
            .value();
        // write_put_uuid(value);
        // log::debug!("Value: {}", value_uuid.clone());

        let _uuid = value_uuid.clone();
        
        if _uuid == uuid_org {
            log::debug!("Deleting Server: {}", _uuid);
            let stable = create_local_resource( move || value_uuid.clone(), 
            move |_uuid| async move { delete_server(_uuid.clone()).await });

            match stable.get() {
                Some(data) => {
                    // view! {
                    //     <div class="toast">
                    //       <div class="alert alert-success">
                    //         <span>New message arrived.</span>
                    //       </div>
                    //     </div>
                    // };
                    log::debug!("Data: {:?}", data);
                },
                None => {
                    // view! {
                    //     <div class="toast">
                    //       <div class="alert alert-error">
                    //         <span>New message arrived.</span>
                    //       </div>
                    //     </div>
                    // };
                    log::debug!("NULL");
                }
            }
        }
    };

    

    view! {
        <form on:submit=on_submit>
            <div class="modal-box w-full">
                <h3 class="font-bold text-sm">Delete Server {name.clone()}.  Type in the UUID to procced.</h3>
                <h3 class="font-bold text-sm">{uuid.clone()}</h3>
                <div class="py-4 grid grid-rows-1 grid-flow-col gap-4">
                    <div>
                        <input type="text" placeholder="UUID" class="input input-bordered w-full max-w-xs" value=read_put_uuid node_ref=input_element_uuid/>
                    </div>
                </div>
                <div class="btn btn-outline btn-warning w-full">
                    This process can NOT be reversed.
                </div>
                <div class="grid grid-rows-1 gap-8">
                    <input class="btn btn-outline btn-error" type="submit" value="DELETE"/>
                </div>
            </div>
        </form>
    }

}