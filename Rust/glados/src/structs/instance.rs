use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rule {

}

#[derive(Serialize, Deserialize, Debug)]
pub struct Instance {
    pub uuid: String,
    pub dest_uuid: String,
    pub pos: Vec3,
    pub rot: Vec2,
    pub rules: Vec<Rule>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InstanceVec {
    pub instances: Vec<Instance>
}