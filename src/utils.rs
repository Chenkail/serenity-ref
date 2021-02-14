use serenity::{model::prelude::*, prelude::*, utils::{parse_channel, parse_username, parse_role}};


/// Searches for a member given the context, message, and user name/mention
pub(crate) async fn get_member(ctx: &Context, msg: &Message, member_name: &str) -> Option<Member> {
    let member_option = parse_username(&member_name);

    let guild_members = ctx.cache.guild_field(msg.guild_id.unwrap(), |guild| guild.members.to_owned()).await.unwrap();
    if let Some(i) = member_option {
        // If user mention found, use it
        let id = UserId::from(i);
        let member_option = guild_members.get(&id);
        if let Some(member) = member_option {
            return Some(member.to_owned());
        } else {
            println!("Couldn't find user: {} in find_member()! Maybe try enabling the Server Members Intent?", member_name);

            return None;
        }
    } else {
        // Otherwise try to match by username
        for (_, member) in guild_members {
            if member.display_name().to_lowercase() == member_name.to_lowercase() || member.display_name().to_lowercase() == member_name.to_lowercase() {
                return Some(member);
            }
        }
    }
    
    return None;
}

/// Searches for a channel given the context, message, and channel name/mention
pub(crate) async fn get_channel(ctx: &Context, msg: &Message, channel_name: &str) -> Option<GuildChannel> {
    let channel_option = parse_channel(&channel_name);
    let channel_list = ctx.cache.guild_channels(msg.guild_id.unwrap()).await.unwrap();
    if let Some(i) = channel_option {
        // If role mention found, use it
        let id = ChannelId::from(i);
        return Some(channel_list.get(&id).unwrap().to_owned());
    } else {
        // Otherwise try to match by channel name
        for (_, channel) in channel_list {
            if channel.name.to_lowercase() == channel_name.to_lowercase() {
                return Some(channel);
            }
        }
    }
    
    return None;
}

/// Searches for a role given the context, message, and role name/mention
pub(crate) async fn get_role(ctx: &Context, msg: &Message, role_name: &str) -> Option<Role> {
    let role_option = parse_role(&role_name);
    let guild_roles = ctx.cache.guild_roles(msg.guild_id.unwrap()).await.unwrap();
    if let Some(i) = role_option {
        // If role mention found, use it
        let id = RoleId::from(i);
        return Some(guild_roles.get(&id).unwrap().to_owned());
    } else {
        // Otherwise try to match by role name
        for (_, role) in guild_roles {
            if role.name.to_lowercase() == role_name.to_lowercase() {
                return Some(role);
            }
        }
    }

    return None;
}
