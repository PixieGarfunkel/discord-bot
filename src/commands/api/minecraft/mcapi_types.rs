use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct MCUserMeta {
    pub cached_at: u32,
}

#[derive(Deserialize, Debug)]
pub struct MCUserProperties {
    pub name: String,
    pub value: String,
    pub signature: String,
}

#[derive(Deserialize, Debug)]
pub struct MCUserNameHistory {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct MCUserPlayer {
    pub meta: MCUserMeta,
    pub username: String,
    pub id: String,
    pub raw_id: String,
    pub avatar: String,
    pub skin_texture: String,
    pub properties: Vec<MCUserProperties>,
    pub name_history: Vec<MCUserNameHistory>,
}

#[derive(Deserialize, Debug)]
pub struct MCUserData {
    pub player: Option<MCUserPlayer>,
}

#[derive(Deserialize, Debug)]
pub struct MCUser {
    pub code: String,
    pub message: String,
    pub data: MCUserData,
    pub success: bool,
    pub error: Option<bool>,
}