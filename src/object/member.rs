pub enum MemberFlags {
    /// Member has left and rejoined the guild.
    DidRejoin = 1<<0,
    /// Member has completed onboarding.
    CompletedOnboarding = 1<<1,
    /// Member is exempt from guild verification requirements.
    BypassesVerification = 1<<2,
    /// Member has started onboarding.
    StartedOnboarding = 1<<3
}

pub enum MemberPermission {
    /// Allows creation of instant invites.
    CreateInstantInvite = 1<<0,
    /// Allows kicking members.
    KickMembers = 1<<1,
    /// Allows banning members.
    BanMembers = 1<<2,
    /// Allows all permissions and bypasses channel permission overwrites.
    Administrator = 1<<3,
    /// Allows management and editing of channels.
    ManageChannels = 1<<4,
    /// Allows management and editing of guild.
    ManageGuild = 1<<5,
    /// Allows for the addition of reactions to messages.
    AddReactions = 1<<6,
    /// Allows for viewing of audit logs.
    ViewAuditLog = 1<<7,
    /// Allows for using priority speaker in a voice channel.
    PrioritySpeaker = 1<<8,
    /// Allows the user to go live.
    Stream = 1<<9,
    /// Allows guild members to view a channel, which includes reading messages
    /// in text channels and joining voice channels.
    ViewChannel = 1<<10,
    /// Allows for sending messages in a channel and creating threads in a forum
    /// (does not allow sending messages in threads).
    SendMessages = 1<<11,
    /// Allows for sending of *"/tts"* messages
    SendTTSMessages = 1<<12,
    /// Allows for deletion of other users messages.
    ManageMessages = 1<<13,
    /// Links sent by users with this permission will be auto-embedded.
    EmbedLinks = 1<<14,
    /// Allows for uploading images and files.
    AttachFiles = 1<<15,
    /// Allows for reading of message history.
    ReadyMessageHistory = 1<<16,
    /// Allows for using the `@everyone` tag to notify all users in a channel, and
    /// the `@here` tag to notify all online users in a channel.
    MentionEveryone = 1<<17,
    /// Allows the usage of custom emojis from other guilds.
    UseExternalEmojis = 1<<18,
    /// Allows for viewing guild insights.
    ViewGuildInsights = 1<<19,
    /// Allows for joining a voice channel.
    Connect = 1<<20,
    /// Allows for speaking in a voice channel.
    Speak = 1<<21,
    /// Allows for muting members in a voice channel.
    MuteMembers = 1<<22,
    /// Allows for deafening members in a voice channel.
    DeafenMembers = 1<<23,
    /// Allows for moving members between voice channels.
    MoveMembers = 1<<24,
    /// Allows for using `voice-activity-detection` in a voice channel.
    UseVAD = 1<<25,
    /// Allows for modification of own name.
    ChangeNickname = 1<<26,
    /// Allows for management of other users nicknames.
    ManageNicknames = 1<<27,
    /// Allows management and editing of role.
    ManageRoles = 1<<28,
    /// Allows management and editing of webhooks.
    ManageWebhooks = 1<<29,
    /// Allows for editing and deleting emojis, stickers, and soundboard sounds
    /// created by all users.
    ManageGuildExpressions = 1<<30,
    /// Allows members to use application commands, including slash commands
    /// and context menu commands.
    UseApplicationCommands = 1<<31,
    /// Allows for requesting to speak in stage channels.
    RequestToSpeak = 1<<32,
    /// Allows for editing and deleting scheduled events created by
    /// all users.
    ManageEvents = 1<<33,
    /// Allows for deleting and archiving threads, and viewing all private threads.
    ManageThreads = 1<<34,
    /// Allows for creating public and announcement threads.
    CreatePublicThreads = 1<<35,
    /// Allows for creating private threads.
    CreatePrivateThreads = 1<<36,
    /// Allows for usage of custom stickers from other guilds.
    UseExternalStickers = 1<<37,
    /// Allows for sending messages in threads.
    SendMessagesInThreads = 1<<38,
    /// Allows for using activities in a voice channel.
    UseEmbeddedActivites = 1<<39,
    /// Allows for timing out users to prevent them from sending or reacting to
    /// messages in chat and threads, and from speaking in voice and stage channels
    ModerateMembers = 1<<40,
    /// Allows for viewing role subscription insights.
    ViewCreatorMonetizationAnalytics = 1<<41,
    /// Allows for using soundboard in a voice channel.
    UseSoundboard = 1<<42,
    /// Allows for creating emojis, stickers, and soundboard sounds, and editing and
    /// deleting those created by the current user.
    CreateGuildExpression = 1<<43,
    /// Allows for creating scheduled events, and editing and deleting those created
    /// by the current user.
    CreateEvents = 1<<44,
    /// Allows the usage of custom soundboard sounds from other guilds.
    UseExternalSounds = 1<<45,
    /// Allows sending voice messages.
    SendVoiceMessages = 1<<46,
    /// Allows sending polls.
    SendPolls = 1<<47
}