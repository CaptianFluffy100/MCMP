use crate::{error_template::{AppError, ErrorTemplate}, structs::{cob::GLaDOSError, portal::PortalVec}, app::{PopulateSideBar, GladosMainBtn}};
use leptos::{*, html::Tr};
use leptos_meta::*;
use leptos_router::*;
use leptos::{error::Result, *};
use serde::{Deserialize, Serialize};
use stylers::style;

use crate::structs::server::ServerVec;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    view! {
        <div class="navbar bg-base-100 h-full" style="height: 100%;">
            <div class="drawer h-full" style="height: 100%;">
              <input id="my-drawer" type="checkbox" class="drawer-toggle" />
              <div class="drawer-content" style="height: 100%;">
                // <div inner-html={page_data}/>
                {GladosMainBtn}
                // {ServerPageDyn}
              </div> 
              {PopulateSideBar}
            </div>
        </div>
    }
}