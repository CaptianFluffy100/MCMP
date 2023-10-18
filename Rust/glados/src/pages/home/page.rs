use crate::{error_template::{AppError, ErrorTemplate}, structs::{cob::GLaDOSError, portal::PortalVec}, app::{PopulateSideBar, GladosMainBtn, get_servers, get_portal_configs, get_portal_instances}};
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
    // Get servers and portal lists before showing the page
    let server_list_data = create_local_resource(|| (), |_| async move { get_servers().await });
    let portal_list_data = create_local_resource(|| (), |_| async move { get_portal_configs().await });
    let portal_instance_data = create_local_resource(|| (), |_| async move { get_portal_instances().await });

    // let mut server_list_size = 0;
    // let mut portal_list_data_size = 0;
    // let mut portal_instance_data_size = 0;

    // match server_list_data {
    //   None => {},
    //   Some(data) => {
    //     server_list_size = data.expect("Error").len();
    //   }
    // }
    // match portal_list_data {
    //   None => {},
    //   Some(data) => {
    //     portal_list_data_size = data.expect("Error").len();
    //   }
    // }
    // match portal_instance_data {
    //   None => {},
    //   Some(data) => {
    //     portal_instance_data_size = data.expect("Error").len();
    //   }
    // }

    view! {
        <div class="navbar bg-base-100 h-full" style="height: 100%;">
            <div class="drawer h-full" style="height: 100%;">
              <input id="my-drawer" type="checkbox" class="drawer-toggle" />
              <div class="drawer-content" style="height: 100%;">
                // <div inner-html={page_data}/>
                {GladosMainBtn}
                // {ServerPageDyn}
                <div class="w-full text-center">
                  // Server info
                  <div class="stats shadow">
      
                    <div class="stat">
                      <div class="stat-figure text-primary">
                      <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="inline-block w-8 h-8 stroke-current"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M18 7H18.01M15 7H15.01M18 17H18.01M15 17H15.01M6 10H18C18.9319 10 19.3978 10 19.7654 9.84776C20.2554 9.64477 20.6448 9.25542 20.8478 8.76537C21 8.39782 21 7.93188 21 7C21 6.06812 21 5.60218 20.8478 5.23463C20.6448 4.74458 20.2554 4.35523 19.7654 4.15224C19.3978 4 18.9319 4 18 4H6C5.06812 4 4.60218 4 4.23463 4.15224C3.74458 4.35523 3.35523 4.74458 3.15224 5.23463C3 5.60218 3 6.06812 3 7C3 7.93188 3 8.39782 3.15224 8.76537C3.35523 9.25542 3.74458 9.64477 4.23463 9.84776C4.60218 10 5.06812 10 6 10ZM6 20H18C18.9319 20 19.3978 20 19.7654 19.8478C20.2554 19.6448 20.6448 19.2554 20.8478 18.7654C21 18.3978 21 17.9319 21 17C21 16.0681 21 15.6022 20.8478 15.2346C20.6448 14.7446 20.2554 14.3552 19.7654 14.1522C19.3978 14 18.9319 14 18 14H6C5.06812 14 4.60218 14 4.23463 14.1522C3.74458 14.3552 3.35523 14.7446 3.15224 15.2346C3 15.6022 3 16.0681 3 17C3 17.9319 3 18.3978 3.15224 18.7654C3.35523 19.2554 3.74458 19.6448 4.23463 19.8478C4.60218 20 5.06812 20 6 20Z"></path></svg>
                      </div>
                      <div class="stat-title">Total Servers</div>
                      <div class="stat-value text-primary">{
                        move ||
                        match server_list_data.get() {
                          None => {
                            0
                          },
                          Some(data) => {
                            data.expect("Error").len()
                          }
                        }
                      }</div>
                      // <div class="stat-desc">21% more than last month</div>
                    </div>

                    <div class="stat">
                      <div class="stat-figure text-secondary">
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="inline-block w-8 h-8 stroke-current"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M1.33309 8.07433C0.92156 8.44266 0.886539 9.07485 1.25487 9.48638C1.62319 9.89791 2.25539 9.93293 2.66691 9.5646L1.33309 8.07433ZM21.3331 9.5646C21.7446 9.93293 22.3768 9.89791 22.7451 9.48638C23.1135 9.07485 23.0784 8.44266 22.6669 8.07433L21.3331 9.5646ZM12 19C11.4477 19 11 19.4477 11 20C11 20.5523 11.4477 21 12 21V19ZM12.01 21C12.5623 21 13.01 20.5523 13.01 20C13.01 19.4477 12.5623 19 12.01 19V21ZM14.6905 17.04C15.099 17.4116 15.7315 17.3817 16.1031 16.9732C16.4748 16.5646 16.4448 15.9322 16.0363 15.5605L14.6905 17.04ZM18.0539 13.3403C18.4624 13.7119 19.0949 13.682 19.4665 13.2734C19.8381 12.8649 19.8082 12.2324 19.3997 11.8608L18.0539 13.3403ZM7.96372 15.5605C7.55517 15.9322 7.52524 16.5646 7.89687 16.9732C8.2685 17.3817 8.90095 17.4116 9.3095 17.04L7.96372 15.5605ZM4.60034 11.8608C4.19179 12.2324 4.16185 12.8649 4.53348 13.2734C4.90511 13.682 5.53756 13.7119 5.94611 13.3403L4.60034 11.8608ZM2.66691 9.5646C5.14444 7.34716 8.41371 6 12 6V4C7.90275 4 4.16312 5.54138 1.33309 8.07433L2.66691 9.5646ZM12 6C15.5863 6 18.8556 7.34716 21.3331 9.5646L22.6669 8.07433C19.8369 5.54138 16.0972 4 12 4V6ZM12 21H12.01V19H12V21ZM12 16C13.0367 16 13.9793 16.3931 14.6905 17.04L16.0363 15.5605C14.9713 14.5918 13.5536 14 12 14V16ZM12 11C14.3319 11 16.4546 11.8855 18.0539 13.3403L19.3997 11.8608C17.4466 10.0842 14.8487 9 12 9V11ZM9.3095 17.04C10.0207 16.3931 10.9633 16 12 16V14C10.4464 14 9.02872 14.5918 7.96372 15.5605L9.3095 17.04ZM5.94611 13.3403C7.54544 11.8855 9.66815 11 12 11V9C9.15127 9 6.55344 10.0842 4.60034 11.8608L5.94611 13.3403Z"></path></svg>
                      </div>
                      <div class="stat-title">Online Servers</div>
                      <div class="stat-value text-secondary">19</div>
                      // <div class="stat-desc">21% more than last month</div>
                    </div>

                    <div class="stat">
                      <div class="stat-figure text-accent">
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="inline-block w-8 h-8 stroke-current"><path d="M18 7.16C17.94 7.15 17.87 7.15 17.81 7.16C16.43 7.11 15.33 5.98 15.33 4.58C15.33 3.15 16.48 2 17.91 2C19.34 2 20.49 3.16 20.49 4.58C20.48 5.98 19.38 7.11 18 7.16Z" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path> <path d="M16.9699 14.44C18.3399 14.67 19.8499 14.43 20.9099 13.72C22.3199 12.78 22.3199 11.24 20.9099 10.3C19.8399 9.59004 18.3099 9.35003 16.9399 9.59003" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path> <path d="M5.96998 7.16C6.02998 7.15 6.09998 7.15 6.15998 7.16C7.53998 7.11 8.63998 5.98 8.63998 4.58C8.63998 3.15 7.48998 2 6.05998 2C4.62998 2 3.47998 3.16 3.47998 4.58C3.48998 5.98 4.58998 7.11 5.96998 7.16Z" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path> <path d="M6.99994 14.44C5.62994 14.67 4.11994 14.43 3.05994 13.72C1.64994 12.78 1.64994 11.24 3.05994 10.3C4.12994 9.59004 5.65994 9.35003 7.02994 9.59003" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path> <path d="M12 14.63C11.94 14.62 11.87 14.62 11.81 14.63C10.43 14.58 9.32996 13.45 9.32996 12.05C9.32996 10.62 10.48 9.46997 11.91 9.46997C13.34 9.46997 14.49 10.63 14.49 12.05C14.48 13.45 13.38 14.59 12 14.63Z" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path> <path d="M9.08997 17.78C7.67997 18.72 7.67997 20.26 9.08997 21.2C10.69 22.27 13.31 22.27 14.91 21.2C16.32 20.26 16.32 18.72 14.91 17.78C13.32 16.72 10.69 16.72 9.08997 17.78Z" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path></svg>
                      </div>
                      <div class="stat-title">Players Online</div>
                      <div class="stat-value text-accent">25,624</div>
                      // <div class="stat-desc text-secondary">31 tasks remaining</div>
                    </div>

                  </div>

                  // Portal Info
                  <div class="stats shadow">
      
                    <div class="stat">
                      <div class="stat-figure text-primary">
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="inline-block w-8 h-8 stroke-current"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 10.4V20M12 10.4C12 8.15979 12 7.03969 11.564 6.18404C11.1805 5.43139 10.5686 4.81947 9.81596 4.43597C8.96031 4 7.84021 4 5.6 4H4.6C4.03995 4 3.75992 4 3.54601 4.10899C3.35785 4.20487 3.20487 4.35785 3.10899 4.54601C3 4.75992 3 5.03995 3 5.6V16.4C3 16.9601 3 17.2401 3.10899 17.454C3.20487 17.6422 3.35785 17.7951 3.54601 17.891C3.75992 18 4.03995 18 4.6 18H7.54668C8.08687 18 8.35696 18 8.61814 18.0466C8.84995 18.0879 9.0761 18.1563 9.29191 18.2506C9.53504 18.3567 9.75977 18.5065 10.2092 18.8062L12 20M12 10.4C12 8.15979 12 7.03969 12.436 6.18404C12.8195 5.43139 13.4314 4.81947 14.184 4.43597C15.0397 4 16.1598 4 18.4 4H19.4C19.9601 4 20.2401 4 20.454 4.10899C20.6422 4.20487 20.7951 4.35785 20.891 4.54601C21 4.75992 21 5.03995 21 5.6V16.4C21 16.9601 21 17.2401 20.891 17.454C20.7951 17.6422 20.6422 17.7951 20.454 17.891C20.2401 18 19.9601 18 19.4 18H16.4533C15.9131 18 15.643 18 15.3819 18.0466C15.15 18.0879 14.9239 18.1563 14.7081 18.2506C14.465 18.3567 14.2402 18.5065 13.7908 18.8062L12 20"></path></svg>
                      </div>
                      <div class="stat-title">Portal Defs</div>
                      <div class="stat-value text-primary">{
                        move ||
                        match portal_list_data.get() {
                          None => {
                            0
                          },
                          Some(data) => {
                            data.expect("Error").len()
                          }
                        }
                      }</div>
                      // <div class="stat-desc">21% more than last month</div>
                    </div>

                    <div class="stat">
                      <div class="stat-figure text-secondary">
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 34 34" class="inline-block w-8 h-8 stroke-current"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M30.135 17.039c0-1.494-3.593-2.777-8.71-3.325-0.009-0.113-0.019-0.226-0.030-0.338-0.002-0.016-0.003-0.031-0.005-0.047-0.011-0.115-0.023-0.229-0.037-0.342-0.002-0.017-0.004-0.033-0.006-0.050-0.013-0.113-0.028-0.225-0.043-0.336-0.002-0.017-0.005-0.034-0.007-0.050-0.016-0.111-0.032-0.221-0.050-0.33-0.001-0.009-0.003-0.018-0.004-0.026l0 0.004c-0.037-0.227-0.079-0.45-0.127-0.668 5.279-0.528 9.017-1.833 9.017-3.357 0-1.724-4.786-3.167-11.177-3.523-0.070-1.785-1.053-3.258-2.362-3.687-0.252-0.082-0.515-0.126-0.787-0.126-0.109 0-0.221-0.014-0.327 0l0 0c-1.537 0.2-2.747 1.826-2.822 3.821-6.314 0.37-11.023 1.804-11.023 3.515-0 1.525 3.741 2.829 9.025 3.357-0.082 0.333-0.153 0.678-0.213 1.031l0-0.003c-0.002 0.010-0.003 0.020-0.005 0.030-0.019 0.115-0.038 0.23-0.055 0.346-0.002 0.016-0.005 0.033-0.007 0.049-0.017 0.115-0.032 0.23-0.047 0.346-0.002 0.018-0.004 0.036-0.007 0.053-0.014 0.112-0.027 0.223-0.039 0.336-5.087 0.551-8.653 1.83-8.653 3.318-0 1.705 4.68 3.135 10.963 3.511l0.12 1.615c-6.354 0.363-11.101 1.801-11.101 3.519 0 1.992 6.38 3.608 14.25 3.608s14.25-1.615 14.25-3.608c0-1.713-4.718-3.148-11.044-3.516l0.107-1.618c6.278-0.377 10.953-1.806 10.953-3.511zM26.054 16.565c0 1.121-2.824 2.081-6.77 2.439l0.075-1.142h2.172c0-0.792 0.003-1.656-0.022-2.535l0 0.001c-0-0.003-0-0.006-0-0.008-0.003-0.121-0.007-0.242-0.012-0.363-0-0.012-0.001-0.023-0.001-0.034-0.005-0.121-0.010-0.241-0.016-0.362-0.001-0.013-0.001-0.025-0.002-0.038-0.002-0.031-0.003-0.062-0.005-0.092 2.76 0.453 4.581 1.235 4.581 2.135zM26.054 7.701c0 0.988-2.194 1.851-5.416 2.287-0.519-1.2-1.338-2.059-2.633-2.26 0.511-0.641 0.862-1.5 0.94-2.461 4.122 0.323 7.108 1.282 7.108 2.435zM5.683 7.701c0-1.139 2.938-2.089 6.992-2.423 0.083 0.982 0.453 1.853 0.986 2.496-1.167 0.22-1.952 1.057-2.48 2.221-3.263-0.433-5.499-1.301-5.499-2.294zM5.683 16.565c0-0.893 1.807-1.67 4.541-2.125-0.003 0.033-0.005 0.065-0.008 0.098-0.002 0.020-0.003 0.039-0.004 0.059-0.008 0.115-0.016 0.23-0.024 0.345-0.001 0.021-0.002 0.042-0.004 0.063-0.007 0.113-0.013 0.226-0.019 0.339-0.001 0.024-0.002 0.049-0.003 0.073-0.005 0.108-0.010 0.216-0.014 0.324-0 0.010-0.001 0.019-0.001 0.029l0-0.004c-0.028 0.724-0.035 1.433-0.035 2.095h2.288l0.085 1.142c-3.956-0.357-6.803-1.318-6.803-2.439zM26.048 25.222c0 1.423-4.557 2.577-10.178 2.577s-10.178-1.154-10.178-2.577c0-1.149 2.971-2.122 7.074-2.455l0.242 3.245c1.537 0.541 4.496 0.482 5.815 0l0.213-3.24c4.071 0.337 7.012 1.306 7.012 2.45z"></path></svg>
                      </div>
                      <div class="stat-title">Total Portals</div>
                      <div class="stat-value text-secondary">{
                        move ||
                        match portal_instance_data.get() {
                          None => {
                            0
                          },
                          Some(data) => {
                            data.expect("Error").len()
                          }
                        }
                      }</div>
                      // <div class="stat-desc">21% more than last month</div>
                    </div>

                    <div class="stat">
                      <div class="stat-figure text-accent">
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="inline-block w-8 h-8 stroke-current"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"></path></svg>
                      </div>
                      <div class="stat-title">Portal Connections</div>
                      <div class="stat-value text-accent">500</div>
                      // <div class="stat-desc text-secondary">31 tasks remaining</div>
                    </div>

                  </div>
                </div>

              </div> 
              {PopulateSideBar}
            </div>
        </div>
    }
}