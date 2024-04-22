use crate::user;

/// [discord docs @ role-object](https://discord.com/developers/docs/topics/permissions#role-object)
#[derive(Debug)]
pub struct Role<'a> {
    pub id:             String,
    pub name:           String,
    pub color:          u32,
    pub hoist:          bool,
    pub icon_hash:      String,

    // emoji

    pub position:       u32,
    pub managed:        bool,
    pub mentionable:    bool,
    pub tags:           RoleTags<'a>, // role tags object
    pub flags:          Vec<RoleFlag>
}

#[derive(Debug)]
pub struct RoleTags<'a> {
    pub bot:                    Option<&'a user::User>,
    pub integration:            Option<String>,
    pub premium_subscriber:     bool,
    pub subscription_listing:   Option<String>,
    pub availabel_for_purchase: bool,
    pub guild_connections:      bool,
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum RoleFlag {
    /// role can be selected by members in an oboarding prompt
    IN_PROMPT = 1<<0
}