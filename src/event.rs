use crate::guild;
use crate::role;
use crate::channel;
use crate::member;
use crate::invite;
use crate::message;

#[allow(unused_variables)]
pub trait EventHandler<'a> {
    fn debug() {}
    fn ready(client: &'a crate::Client<'a>){}
    fn guild_create(client: &'a crate::Client<'a>, guild: &'a guild::Guild<'a>){}
    fn guild_update(client: &'a crate::Client<'a>, guild: &'a guild::Guild<'a>, old_guild: guild::Guild<'a>){}
    fn guild_delete(client: &'a crate::Client<'a>, guild: guild::Guild<'a>){}
    fn guild_role_create(client: &'a crate::Client<'a>, role: &'a role::Role<'a>){}
    fn guild_role_update(client: &'a crate::Client<'a>, role: &'a role::Role<'a>, old_role: role::Role<'a>){}
    fn guild_role_delete(client: &'a crate::Client<'a>, role: role::Role<'a>){}
    fn channel_create(client: &'a crate::Client<'a>, channel: &'a channel::Channel<'a>){}
    fn channel_update(client: &'a crate::Client<'a>, channel: &'a channel::Channel<'a>, old_channel: channel::Channel<'a>){}
    fn channel_delete(client: &'a crate::Client<'a>, channel: channel::Channel<'a>){}
    fn channel_pins_update(client: &'a crate::Client<'a>){} // wip
    // not sure how threads are going to work yet
    // thread_create
    // thread_update
    // thread_delete
    // thread_list_sync
    // thread_member_update
    // thread_members_update
    fn stage_instance_create(client: &'a crate::Client<'a>){} // wip
    fn stage_instance_update(client: &'a crate::Client<'a>){} // wip
    fn stage_instance_delete(client: &'a crate::Client<'a>){} // wip
    fn guild_member_add(client: &'a crate::Client<'a>, member: &'a member::Member<'a>){}
    fn guild_member_update(client: &'a crate::Client<'a>, member: &'a member::Member<'a>, old_member: member::Member<'a>){}
    fn guild_member_remove(client: &'a crate::Client<'a>, member: member::Member<'a>){}
    fn guild_audit_log_entry_create(client: &'a crate::Client<'a>){} // wip
    fn guild_ban_add(client: &'a crate::Client<'a>){} // wip
    fn guild_ban_remove(client: &'a crate::Client<'a>){} // wip
    fn guild_emojis_update(client: &'a crate::Client<'a>){} // wip
    fn guild_stickers_update(client: &'a crate::Client<'a>){} // wip
    fn guild_integrations_update(client: &'a crate::Client<'a>){} // wip
    fn integration_create(client: &'a crate::Client<'a>){} // wip
    fn integration_update(client: &'a crate::Client<'a>){} // wip
    fn integration_delete(client: &'a crate::Client<'a>){} // wip
    fn webhooks_update(client: &'a crate::Client<'a>){} // wip
    fn invite_create(client: &'a crate::Client<'a>, invite: &'a invite::Invite<'a>){}
    fn invite_delete(client: &'a crate::Client<'a>, invite: invite::Invite<'a>){}
    fn presence_update(client: &'a crate::Client<'a>){} // wip
    fn message_create(client: &'a crate::Client<'a>, message: &'a message::Message<'a>){}
    fn message_update(client: &'a crate::Client<'a>, message: &'a message::Message<'a>, old_message: message::Message<'a>){}
    fn message_delete(client: &'a crate::Client<'a>, message: message::Message<'a>){}
    fn message_delete_bulk(client: &'a crate::Client<'a>){} // wip
    fn message_reaction_add(client: &'a crate::Client<'a>){} // wip
    fn message_reaction_remove(client: &'a crate::Client<'a>){} // wip
    fn message_reaction_remove_all(client: &'a crate::Client<'a>){} // wip
    fn message_reaction_remove_emoji(client: &'a crate::Client<'a>){} // wip
    fn typing_start(client: &'a crate::Client<'a>){} // wip
    fn guild_scheduled_event_create(client: &'a crate::Client<'a>){} // wip
    fn guild_scheduled_event_update(client: &'a crate::Client<'a>){} // wip
    fn guild_scheduled_event_delete(client: &'a crate::Client<'a>){} // wip
    fn guild_scheduled_event_user_add(client: &'a crate::Client<'a>){} // wip
    fn guild_scheduled_event_user_remove(client: &'a crate::Client<'a>){} // wip
    fn auto_moderation_rule_create(client: &'a crate::Client<'a>){} // wip
    fn auto_moderation_rule_update(client: &'a crate::Client<'a>){} // wip
    fn auto_moderation_rule_delete(client: &'a crate::Client<'a>){} // wip
    fn auto_moderation_action_execution(client: &'a crate::Client<'a>){} // wip
    fn message_poll_vote_add(client: &'a crate::Client<'a>){} // wip
    fn message_poll_vote_remove(client: &'a crate::Client<'a>){} // wip
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Event {
    GUILD_CREATE,
    GUILD_UPDATE,
    GUILD_DELETE,
    GUILD_ROLE_CREATE,
    GUILD_ROLE_UPDATE,
    GUILD_ROLE_DELETE,
    CHANNEL_CREATE,
    CHANNEL_UPDATE,
    CHANNEL_DELETE,
    CHANNEL_PINS_UPDATE,
    THREAD_CREATE,
    THREAD_UPDATE,
    THREAD_DELETE,
    THREAD_LIST_SYNC,
    THREAD_MEMBER_UPDATE,
    THREAD_MEMBERS_UPDATE,
    STAGE_INSTANCE_CREATE,
    STAGE_INSTANCE_UPDATE,
    STAGE_INSTANCE_DELETE,
    GUILD_MEMBER_ADD,
    GUILD_MEMBER_UPDATE,
    GUILD_MEMBER_REMOVE,
    GUILD_AUDIT_LOG_ENTRY_CREATE,
    GUILD_BAN_ADD,
    GUILD_BAN_REMOVE,
    GUILD_EMOJIS_UPDATE,
    GUILD_STICKERS_UPDATE,
    GUILD_INTEGRATIONS_UPDATE,
    INTEGRATION_CREATE,
    INTEGRATION_UPDATE,
    INTEGRATION_DELETE,
    WEBHOOKS_UPDATE,
    INVITE_CREATE,
    INVITE_DELETE,
    VOICE_STATE_UPDATE,
    PRESENCE_UPDATE,
    MESSAGE_CREATE,
    MESSAGE_UPDATE,
    MESSAGE_DELETE,
    MESSAGE_DELETE_BULK,
    MESSAGE_REACTION_ADD,
    MESSAGE_REACTION_REMOVE,
    MESSAGE_REACTION_REMOVE_ALL,
    MESSAGE_REACTION_REMOVE_EMOJI,
    TYPING_START,
    GUILD_SCHEDULED_EVENT_CREATE,
    GUILD_SCHEDULED_EVENT_UPDATE,
    GUILD_SCHEDULED_EVENT_DELETE,
    GUILD_SCHEDULED_EVENT_USER_ADD,
    GUILD_SCHEDULED_EVENT_USER_REMOVE,
    AUTO_MODERATION_RULE_CREATE,
    AUTO_MODERATION_RULE_UPDATE,
    AUTO_MODERATION_RULE_DELETE,
    AUTO_MODERATION_ACTION_EXECUTION,
    MESSAGE_POLL_VOTE_ADD,
    MESSAGE_POLL_VOTE_REMOVE
}