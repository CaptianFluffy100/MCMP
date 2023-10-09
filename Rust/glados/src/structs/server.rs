use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Server {
    #[serde(default)]
    pub ip: String,
    pub port: u16,
    pub name: String,
    pub uuid: String
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ServerVec {
    #[serde(default)]
    pub servers: Vec<Server>
}