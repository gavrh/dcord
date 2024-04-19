#[derive(Debug)]
pub struct ClientUser {
    pub id:         String,
    pub username:   String,
}

#[derive(Debug)]
pub struct User {
    pub id:         String,
    pub username:   String,
    pub persona:    String,
    pub flags:      Vec<Flag>
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Flag {
    STAFF                   = 1<<0,
    PARTNER                 = 1<<1,
    HYPESQUAD_EVENTS        = 1<<2,
    BUG_HUNTER              = 1<<3,
    HYPESQUAD_BRAVERY       = 1<<6,
    HYPESQUAD_BRLLIANCE     = 1<<7,
    HYPESQUAD_BALANCE       = 1<<8,
    PREMIUM_EARLY_SUPPORTER = 1<<9,
    TEAM_PSEUDO_USER        = 1<<10,
    BUG_HUNTER_GOLD         = 1<<14,
    VERIFIED_BOT            = 1<<16,
    VERIFIED_DEVELOPER      = 1<<17,
    CERTIFIED_MODERATOR     = 1<<18,
    BOT_HTTP_INTERACTIONS   = 1<<19,
    ACTIVE_DEVELOPER        = 1<<22
}