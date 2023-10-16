use crate::{error_template::{AppError, ErrorTemplate}, structs::{cob::GLaDOSError, portal::PortalVec}, app::{PopulateSideBar, GladosMainBtn, post_portal_config, get_portal_configs, delete_portal_config, put_portal_config}, api::schema::PortalConfig};
use leptos::{*, html::{Tr, Dialog, Input}, ev::SubmitEvent};
use leptos_meta::*;
use leptos_router::*;
use leptos::{error::Result, *};
use serde::{Deserialize, Serialize};
use stylers::style;

use crate::structs::server::ServerVec;

#[component]
pub fn PortalPageEdit() -> impl IntoView {
    let (read_put_id, write_put_id) = create_signal("".to_string());
    let (read_put_name, write_put_name) = create_signal("".to_string());
    let (read_put_frame, write_put_frame) = create_signal("".to_string());
    let (read_put_light, write_put_light) = create_signal("".to_string());
    let (read_put_light_id, write_put_light_id) = create_signal("".to_string());
    let (read_put_color_r, write_put_color_r) = create_signal(0);
    let (read_put_color_g, write_put_color_g) = create_signal(0);
    let (read_put_color_b, write_put_color_b) = create_signal(0);

    let input_element_id: NodeRef<Input> = create_node_ref();
    let input_element_name: NodeRef<Input> = create_node_ref();
    let input_element_frame: NodeRef<Input> = create_node_ref();
    let input_element_light: NodeRef<Input> = create_node_ref();
    let input_element_light_id: NodeRef<Input> = create_node_ref();
    let input_element_color_r: NodeRef<Input> = create_node_ref();
    let input_element_color_g: NodeRef<Input> = create_node_ref();
    let input_element_color_b: NodeRef<Input> = create_node_ref();

    let on_submit = move |ev: SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        // here, we'll extract the value from the input
        let value_id = input_element_id()
            .expect("<input> to exist")
            .value();
        let value_name = input_element_name()
            .expect("<input> to exist")
            .value();
        let value_frame = input_element_frame()
            .expect("<input> to exist")
            .value();
        let value_light = input_element_light()
            .expect("<input> to exist")
            .value();
        let value_light_id = input_element_light_id()
            .expect("<input> to exist")
            .value();
        let value_color_r: u8 = input_element_color_r()
            .expect("<input> to exist")
            .value()
            .parse()
            .unwrap();
        let value_color_g: u8 = input_element_color_g()
            .expect("<input> to exist")
            .value()
            .parse()
            .unwrap();
        let value_color_b: u8 = input_element_color_b()
            .expect("<input> to exist")
            .value()
            .parse()
            .unwrap();
        // write_put_id(value);
        log::debug!("Value: {}", value_id.clone());
        // log::debug!("Value: {}", value_frame.clone());
        // log::debug!("Value: {}", value_light.clone());
        // log::debug!("Value: {}", value_port.clone());

        // let _id = value_id.clone();
        // let _light = value_light.clone();
        // let _port = value_port.clone();
        // let _frame = value_frame.clone();
        
        let stable = create_local_resource( move || (value_id.clone(),value_name.clone(),value_light.clone(),value_light_id.clone(),value_frame.clone(),value_color_r.clone(),value_color_g.clone(),value_color_b.clone()), 
        move |(_id, _name, _light, _light_id, _frame, _color_r, _color_g, _color_b)| async move { post_portal_config(_id, _name, _light, _light_id, _frame, _color_r, _color_g, _color_b).await });

        
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
                            <div class="py-4 grid grid-rows-4 grid-flow-col gap-4">
                            <div>
                                <a>ID: </a>
                                <input type="text" placeholder="ID" class="input input-bordered w-full max-w-xs" value=read_put_id node_ref=input_element_id/>
                            </div>
                            <div>
                                <a>NAME: </a>
                                <input type="text" placeholder="NAME" class="input input-bordered w-full max-w-xs" value=read_put_name node_ref=input_element_name/>
                            </div>
                            <div>
                                <a>FRAME BLOCK: </a>
                                <input type="text" placeholder="FRAME" class="input input-bordered w-full max-w-xs" value=read_put_frame node_ref=input_element_frame />
                            </div>
                            <div>
                                <a>LIGHT WITH (Fire, Item, Fluid)[Item and Fluid need ID]: </a>
                                <input type="text" placeholder="fire, item, or fluid" class="input input-bordered w-full max-w-xs" value=read_put_light node_ref=input_element_light />
                            </div>
                            <div>
                                <a>LIGHT WITH ITEM/FLUID ID: </a>
                                <input type="text" placeholder="item, or fluid id" class="input input-bordered w-full max-w-xs" value=read_put_light_id node_ref=input_element_light_id />
                            </div>
                            <div>
                                <a>COLOR RED: </a>
                                <input type="text" placeholder="COLOR RED" class="input input-bordered w-full max-w-xs" value=read_put_color_r node_ref=input_element_color_r />
                            </div>
                            <div>
                                <a>COLOR GREEN: </a>
                                <input type="text" placeholder="COLOR GREEN" class="input input-bordered w-full max-w-xs" value=read_put_color_g node_ref=input_element_color_g />
                            </div>
                            <div>
                                <a>COLOR BLUE: </a>
                                <input type="text" placeholder="COLOR BLUE" class="input input-bordered w-full max-w-xs" value=read_put_color_b node_ref=input_element_color_b />
                            </div>
                        </div>
                        <div class="grid grid-rows-1 gap-4">
                            <input class="btn btn-outline btn-success" type="submit" on:submit=on_submit value="SAVE"/>
                        </div>
                        </div>
                    </form>
                </dialog>
                {PortalPageEditDyn}
              </div> 
              {PopulateSideBar}
            </div>
        </div>
    }
}

#[component]
pub fn portal_page_edit_dyn() -> impl IntoView {
    let async_data: Resource<(), std::result::Result<Vec<PortalConfig>, error::Error>> = create_local_resource(
        // the first is the "source signal"
        || (),
        // the second is the loader
        // it takes the source signal's value as its argument
        // and does some async work
        |_| async move { get_portal_configs().await },
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
                                    <th>Frame Block</th>
                                    <th>Ligth With</th>
                                    <th>Ligth With ID</th>
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
                                    let mut remove: Vec<HtmlElement<Dialog>> = vec![];

                                    match a {
                                        Ok(data) => {
                                            for portals in data.clone() {
                                                let (ignite_with, ignite_with_id) = portals.ignite_with.deconstruct();
                                                let open_edit = "edit".to_string()+&portals.id.to_string().clone().replace("-", "")+".showModal()";
                                                let open_remove = "remove".to_string()+&portals.id.to_string().clone().replace("-", "")+".showModal()";
                                                html.push(view! {<tr><th>{portals.name.clone()}</th><td>{portals.id.to_string().clone()}</td><td>{portals.frame_block_id.clone()}</td><td>{ignite_with.clone()}</td><td>{ignite_with_id.clone()}</td><td>{portals.color.blue}</td><td>{portals.color.green}</td><td>{portals.color.red}</td><td><a class="btn btn-outline btn-warning" onclick={open_edit}>EDIT</a></td><td><a class="btn btn-outline btn-error" onclick={open_remove}>REMOVE</a></td></tr>});
                                                let edit_props = PortalPageEditFormsDynProps{id: portals.id.to_string().clone(), frame: portals.frame_block_id.clone(), name: portals.name.clone(), color_r:portals.color.red.clone(), color_g: portals.color.green.clone(), color_b: portals.color.blue.clone(), light: ignite_with.clone(), light_id: ignite_with_id.clone()};
                                                edit.push(view! {<dialog id={"edit".to_string()+&portals.id.to_string().replace("-", "").clone()} class="modal">{edit_props}</dialog>});
                                                let remove_props = PortalPageRemoveFormsDynProps{id: portals.id.to_string().clone()};
                                                remove.push(view! {<dialog id={"remove".to_string()+&portals.id.to_string().replace("-", "").clone()} class="modal">{PortalPageRemoveFormsDyn(remove_props)}</dialog>});
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
pub fn portal_page_remove_forms_dyn(id: String) -> impl IntoView {
    let (read_put_id, write_put_id) = create_signal("".to_string());

    let input_element_id: NodeRef<Input> = create_node_ref();

    let id_org = id.clone();

    let on_submit = move |ev: SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        // here, we'll extract the value from the input
        let value_id = input_element_id()
            .expect("<input> to exist")
            .value();
        // write_put_id(value);
        // log::debug!("Value: {}", value_id.clone());

        let _id = value_id.clone();
        
        if _id == id_org {
            log::debug!("Deleting Server: {}", _id);
            let stable = create_local_resource( move || value_id.clone(), 
            move |_id| async move { delete_portal_config(_id.clone()).await });

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
                <h3 class="font-bold text-sm">Delete Portal Definition {id.clone()}.  Type in the ID to procced.</h3>
                <div class="py-4 grid grid-rows-1 grid-flow-col gap-4">
                    <div>
                        <input type="text" placeholder="ID" class="input input-bordered w-full max-w-xs" value=read_put_id node_ref=input_element_id/>
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

#[component]
pub fn portal_page_edit_forms_dyn(id: String, name: String, frame: String, light: String, light_id: String, color_r: u8, color_g: u8, color_b: u8) -> impl IntoView {
    let (read_put_id, write_put_id) = create_signal(id.clone());
    let (read_put_frame, write_put_frame) = create_signal(frame.clone());
    let (read_put_name, write_put_name) = create_signal(name.clone());
    let (read_put_light, write_put_light) = create_signal(light.clone());
    let (read_put_light_id, write_put_light_id) = create_signal(light_id.clone());
    let (read_put_color_r, write_put_color_r) = create_signal(color_r);
    let (read_put_color_g, write_put_color_g) = create_signal(color_g);
    let (read_put_color_b, write_put_color_b) = create_signal(color_b);

    let input_element_id: NodeRef<Input> = create_node_ref();
    let input_element_frame: NodeRef<Input> = create_node_ref();
    let input_element_name: NodeRef<Input> = create_node_ref();
    let input_element_light: NodeRef<Input> = create_node_ref();
    let input_element_light_id: NodeRef<Input> = create_node_ref();
    let input_element_color_r: NodeRef<Input> = create_node_ref();
    let input_element_color_g: NodeRef<Input> = create_node_ref();
    let input_element_color_b: NodeRef<Input> = create_node_ref();

    let on_submit = move |ev: SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        // here, we'll extract the value from the input
        let value_id = input_element_id()
            .expect("<input> to exist")
            .value();
        let value_frame = input_element_frame()
            .expect("<input> to exist")
            .value();
        let value_name = input_element_name()
            .expect("<input> to exist")
            .value();
        let value_light = input_element_light()
            .expect("<input> to exist")
            .value();
        let value_light_id = input_element_light_id()
            .expect("<input> to exist")
            .value();
        let value_color_r: u8 = input_element_color_r()
            .expect("<input> to exist")
            .value()
            .parse()
            .unwrap();
        let value_color_g: u8 = input_element_color_g()
            .expect("<input> to exist")
            .value()
            .parse()
            .unwrap();
        let value_color_b: u8 = input_element_color_b()
            .expect("<input> to exist")
            .value()
            .parse()
            .unwrap();
        // write_put_id(value);
        log::debug!("Value: {}", value_id.clone());
        // log::debug!("Value: {}", value_frame.clone());
        // log::debug!("Value: {}", value_name.clone());
        // log::debug!("Value: {}", value_port.clone());

        // let _id = value_id.clone();
        // let _name = value_name.clone();
        // let _port = value_port.clone();
        // let _frame = value_frame.clone();
        
        let stable = create_local_resource( move || (value_id.clone(),value_name.clone(),value_light.clone(),value_light_id.clone(),value_frame.clone(),value_color_r.clone(),value_color_g.clone(),value_color_b.clone()), 
        move |(_id, _name, _light, _light_id, _frame, _color_r, _color_g, _color_b)| async move { put_portal_config(_id, _name, _light, _light_id, _frame, _color_r, _color_g, _color_b).await });

        
    };

    

    view! {
        <form on:submit=on_submit>
            <div class="modal-box">
                <h3 class="font-bold text-lg">Edit {id.clone()}</h3>
                <div class="py-4 grid grid-rows-4 grid-flow-col gap-4">
                    <div>
                        <a>ID: </a>
                        <input type="text" placeholder="ID" class="input input-bordered w-full max-w-xs" value=read_put_id node_ref=input_element_id disabled/>
                    </div>
                    <div>
                        <a>FRAME BLOCK: </a>
                        <input type="text" placeholder="FRAME" class="input input-bordered w-full max-w-xs" value=read_put_frame node_ref=input_element_frame />
                    </div>
                    <div>
                        <a>NAME: </a>
                        <input type="text" placeholder="name" class="input input-bordered w-full max-w-xs" value=read_put_name node_ref=input_element_name />
                    </div>
                    <div>
                        <a>LIGHT WITH (Fire, Item, Fluid)[Item and Fluid need ID]: </a>
                        <input type="text" placeholder="fire, item, or fluid" class="input input-bordered w-full max-w-xs" value=read_put_light node_ref=input_element_light />
                    </div>
                    <div>
                        <a>LIGHT WITH ITEM/FLUID ID: </a>
                        <input type="text" placeholder="item, or fluid id" class="input input-bordered w-full max-w-xs" value=read_put_light_id node_ref=input_element_light_id />
                    </div>
                    <div>
                        <a>COLOR RED: </a>
                        <input type="text" placeholder="COLOR RED" class="input input-bordered w-full max-w-xs" value=read_put_color_r node_ref=input_element_color_r />
                    </div>
                    <div>
                        <a>COLOR GREEN: </a>
                        <input type="text" placeholder="COLOR GREEN" class="input input-bordered w-full max-w-xs" value=read_put_color_g node_ref=input_element_color_g />
                    </div>
                    <div>
                        <a>COLOR BLUE: </a>
                        <input type="text" placeholder="COLOR BLUE" class="input input-bordered w-full max-w-xs" value=read_put_color_b node_ref=input_element_color_b />
                    </div>
                </div>
                <div class="grid grid-rows-1 gap-4">
                    <input class="btn btn-outline btn-success" type="submit" on:submit=on_submit value="SAVE"/>
                </div>
            </div>
        </form>
    }

}
