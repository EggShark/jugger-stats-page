//! these types all mirror what a DB row looks like

pub struct Team {
    id: u32,
    name: String,
    current_location: String,
    description: Option<String>,
    captitan_id: u32,
}

pub struct Player {
    id: u32,
    name: String,
    user_id: Option<u32>,
}
