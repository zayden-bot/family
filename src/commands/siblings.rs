use async_trait::async_trait;
use futures::{stream, StreamExt, TryStreamExt};
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateEmbed, Mentionable, ResolvedOption, ResolvedValue, UserId,
};
use zayden_core::SlashCommand;

use crate::{Error, Result, utils::{embed_response, message_response}};
use crate::sqlx_lib::PostgresPool;

use super::FamilyRow;

pub struct Siblings;

#[async_trait]
impl SlashCommand<Error> for Siblings {
    async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
        let user = match interaction.data.options().first() {
            Some(ResolvedOption {
                value: ResolvedValue::User(user, _),
                ..
            }) => *user,
            _ => &interaction.user,
        };

        let row = FamilyRow::safe_get(ctx, user.id).await?;

        if row.parent_ids.is_empty() {
            if user == &interaction.user {
                message_response(ctx, interaction, "You have no siblings").await?;
                return Ok(());
            }

            message_response(
                ctx,
                interaction,
                format!("{} has no siblings", user.mention()),
            )
            .await?;
            return Ok(());
        }

        let pool = PostgresPool::get(ctx).await;
        let row = FamilyRow::safe_get(ctx, user.id).await?;

        let sibling_ids = row.sibling_ids(&pool).try_collect::<Vec<_>>().await?;

        let sibling_names = stream::iter(sibling_ids)
            .then(|id| async move {
                let user_id = UserId::new(id as u64);

                if let Some(guild_id) = interaction.guild_id {
                    if let Ok(member) = guild_id.member(ctx, user_id).await {
                        return Ok(member.user.mention().to_string());
                    }
                }

                user_id.to_user(ctx).await.map(|user| user.name)
            })
            .try_collect::<Vec<_>>()
            .await?;

        let siblings_plural = if sibling_names.len() == 1 {
            "sibling"
        } else {
            "siblings"
        };

        let desc = if user == &interaction.user {
            format!(
                "You have {} {}:\n{}",
                sibling_names.len(),
                siblings_plural,
                sibling_names.join("\n")
            )
        } else {
            format!(
                "{} has {} {}:\n{}",
                user.mention(),
                sibling_names.len(),
                siblings_plural,
                sibling_names.join("\n")
            )
        };

        embed_response(ctx, interaction, CreateEmbed::new().description(desc)).await?;

        Ok(())
    }

    fn register() -> CreateCommand {
        CreateCommand::new("siblings")
            .description("List who your siblings are.")
            .add_option(CreateCommandOption::new(
                CommandOptionType::User,
                "user",
                "The user to check. Leave blank to check yourself.",
            ))
    }
}
