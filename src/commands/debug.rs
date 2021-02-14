// use crate::utils::*;

use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::framework::standard::{macros::command, CommandResult};

/// Print out a list of all members in the server
#[command]
pub async fn list(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_members = ctx.cache.guild_field(msg.guild_id.unwrap(), |guild| guild.members.to_owned()).await.unwrap();
    for (id, member) in guild_members {
        println!("User: {}, ID: {}", member.display_name(), id);
    }

    Ok(())
}

/// Print out a list of all roles in the server
#[command]
pub async fn list_roles(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_roles = ctx.cache.guild_roles(msg.guild_id.unwrap()).await.unwrap();
    for (id, role) in guild_roles {
        println!("Role: {}, ID: {}", role.name, id);
    }

    Ok(())
}

/// Print out a list of all channels in the server
#[command]
pub async fn list_channels(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_channels = ctx.cache.guild_channels(msg.guild_id.unwrap()).await.unwrap();
    for (id, channel) in guild_channels {
        println!("Channel: {}, ID: {}", channel.name, id);
    }

    Ok(())
}
