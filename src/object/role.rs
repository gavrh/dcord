pub struct Role {
    ///
    pub id: String,
    pub name: String,
    pub color: u32,
    pub hoist: bool,
    pub icon: Option<String>,
    pub unicode_emoji: Option<String>,
    pub position: u16,
    pub permissions: String, // replace later
    pub managed: bool,
    pub mentionable: bool,
    pub tags: Option<RoleTags>,
    pub flags: Vec<RoleFlag> // replace later
}

pub struct RoleTags {
    // finish later
}

pub enum RoleFlag {
    InPrompt = 1
}