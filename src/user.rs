#[derive(Debug)]
pub struct ClientUser {
    /// client's ID
    pub id:                     String,
    /// client's username, not unique across the platform
    pub username:               String,
    /// client's discord tag
    pub discriminator:          String,
    /// client's display name if set
    pub name:                   String,
    /// client's avatar hash
    pub avatar_hash:            String,
    /// user's banner hash
    pub banner_hash:            String,
    /// the user's banner color (hex)
    pub accent_color:           i32,
    ///
    // locale
    /// whether the email on this account has been verified
    pub verified:               bool,
    
    // email?

    pub flags:                  Vec<UserFlag>,
    pub premium_kind:           UserPremiumKind,
 
    // pub public_flags:        Vec<?PUBLIC?>,

    /// user's avatar decoration hash
    pub avatar_decoration_hash: String,
}

#[derive(Debug)]
pub struct User {
    /// user's ID
    pub id:                     String,
    /// user's username, not unique across the platform
    pub username:               String,
    /// user's discord tag
    pub discriminator:          String,
    /// user's display name if set
    /// 
    /// For bots, this is the application name
    pub name:                   String,
    /// user's avatar hash
    pub avatar_hash:            String,
    /// whether the user belongs to an OAuth application
    pub bot:                    bool,
    /// whether the user is an official discord system user
    pub system:                 bool,
    /// whether the user has two factor enabled on their account
    pub mfa:                    bool,
    /// user's banner hash
    pub banner_hash:            String,
    /// the user's banner color (hex)
    pub accent_color:           i32,
    ///
    // locale
    /// whether the email on this account has been verified
    pub verified:               bool,
    
    // email?

    pub flags:                  Vec<UserFlag>,
    pub premium_kind:           UserPremiumKind,
 
    // pub public_flags:        Vec<?PUBLIC?>,

    /// user's avatar decoration hash
    pub avatar_decoration_hash: String,
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum UserFlag {
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

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum UserPremiumKind {
    NONE,
    NITRO_CLASSIC,
    NITRO,
    NITRO_BASIC
}