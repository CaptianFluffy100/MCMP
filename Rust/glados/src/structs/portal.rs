use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Portal {
    pub index: String,
    pub frameBlockId: String,
    pub lightWithItemId: String,
    pub color_r: u8,
    pub color_g: u8,
    pub color_b: u8,
    // pub command: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PortalVec {
    pub portals: Vec<Portal>
}