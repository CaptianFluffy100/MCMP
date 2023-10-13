use std::{env, fs::{self, File}, io::{Write, Read}, path::PathBuf};

use crate::structs::{instance::InstanceVec, portal::PortalVec, server::ServerVec};

#[cfg(target_family = "windows")]
fn split_value() -> &'static str {
    "\\"
}

#[cfg(not(target_family = "windows"))]
fn split_value() -> &'static str {
    "/"
}

pub fn check_if_file_exists() {
    let path_config = env::current_exe().unwrap();
    println!("{:?}", path_config);
    let path: std::borrow::Cow<'_, str> = path_config.to_string_lossy();
    let mut path: Vec<&str> = path.split(split_value()).collect();
    path.remove(path.len()-1);
    path.push("config");
    let mut path_config_servers = path.clone();
    let mut path_config_portals = path.clone();
    let mut path_config_portal_instances = path.clone();

    path_config_servers.push("servers.json");
    path_config_portals.push("portals.json");
    path_config_portal_instances.push("portal_instances.json");
    

    let mut path_new: String = "".to_string();
    for i in path {
        path_new = path_new + i + "/";
    }
    let path: PathBuf = PathBuf::from(path_new);
    let mut path_config_servers_new: String = "".to_string();
    let mut path_config_portals_new: String = "".to_string();
    let mut path_config_portal_instances_new: String = "".to_string();
    for i in path_config_servers {
        path_config_servers_new = path_config_servers_new + i + "/";
    }
    for i in path_config_portals {
        path_config_portals_new = path_config_portals_new + i + "/";
    }
    for i in path_config_portal_instances {
        path_config_portal_instances_new = path_config_portal_instances_new + i + "/";
    }

    let mut path_config_servers: PathBuf = PathBuf::from(path_config_servers_new);
    let mut path_config_portals: PathBuf = PathBuf::from(path_config_portals_new);
    let mut path_config_portal_instances: PathBuf = PathBuf::from(path_config_portal_instances_new);

    println!("Path: {:?}", path);
    if !path.exists() {
        println!("Create Config file");
        let done = fs::create_dir_all(path);
        println!("Done: {:?}", done);
    }

    if !path_config_servers.exists() {
        // Create File
        let mut file = File::create(path_config_servers).unwrap();
        file.write_all(b"{\"servers\": []}");
    }

    if !path_config_portals.exists() {
        // Create File
        let mut file = File::create(path_config_portals).unwrap();
        file.write_all(b"{\"portals\": []}");
    }

    if !path_config_portal_instances.exists() {
        // Create File
        let mut file = File::create(path_config_portal_instances).unwrap();
        file.write_all(b"{\"instances\": []}");
    }
}

pub async fn get_json_servers() -> ServerVec {
    let mut servers: ServerVec = ServerVec {servers: vec![]};
    println!("Get Server Data");
    let path_config = env::current_exe().unwrap();
    let path: std::borrow::Cow<'_, str> = path_config.to_string_lossy();
    let mut path: Vec<&str> = path.split(split_value()).collect();
    path.remove(path.len()-1);
    path.push("config");
    let mut path_config_servers = path.clone();

    path_config_servers.push("servers.json");

    let mut path_config_servers_new: String = "".to_string();
    for i in path_config_servers {
        path_config_servers_new = path_config_servers_new + i + "/";
    }

    let mut path_config_servers: PathBuf = PathBuf::from(path_config_servers_new);

    if path_config_servers.exists() {
        // Create File
        let mut file = File::open(path_config_servers).unwrap();
        let mut buf = "".to_string();
        let data = file.read_to_string(&mut buf);
        servers = serde_json::from_str(&buf).unwrap();
    }

    servers
}

pub async fn get_json_portals() -> PortalVec {
    let mut portals: PortalVec = PortalVec {portals: vec![]};
    println!("Get Portal Data");
    let path_config = env::current_exe().unwrap();
    let path: std::borrow::Cow<'_, str> = path_config.to_string_lossy();
    let mut path: Vec<&str> = path.split(split_value()).collect();
    path.remove(path.len()-1);
    path.push("config");
    let mut path_config_portals = path.clone();

    path_config_portals.push("portals.json");

    let mut path_config_portals_new: String = "".to_string();
    for i in path_config_portals {
        path_config_portals_new = path_config_portals_new + i + "/";
    }

    let mut path_config_portals: PathBuf = PathBuf::from(path_config_portals_new);

    if path_config_portals.exists() {
        // Create File
        let mut file = File::open(path_config_portals).unwrap();
        let mut buf = "".to_string();
        let data = file.read_to_string(&mut buf);
        portals = serde_json::from_str(&buf).unwrap();
    }
    println!("{:?}", portals);
    portals
}

pub async fn get_json_instances() -> InstanceVec {
    let mut instances: InstanceVec = InstanceVec {instances: vec![]};
    println!("Get Instance Data");
    let path_config = env::current_exe().unwrap();
    let path: std::borrow::Cow<'_, str> = path_config.to_string_lossy();
    let mut path: Vec<&str> = path.split(split_value()).collect();
    path.remove(path.len()-1);
    path.push("config");
    let mut path_config_instances = path.clone();

    path_config_instances.push("portal_instances.json");

    let mut path_config_instances_new: String = "".to_string();
    for i in path_config_instances {
        path_config_instances_new = path_config_instances_new + i + "/";
    }

    let mut path_config_instances: PathBuf = PathBuf::from(path_config_instances_new);

    if path_config_instances.exists() {
        // Create File
        let mut file = File::open(path_config_instances).unwrap();
        let mut buf = "".to_string();
        let data = file.read_to_string(&mut buf);
        instances = serde_json::from_str(&buf).unwrap();
    }

    instances
}