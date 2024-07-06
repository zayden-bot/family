use async_trait::async_trait;
use futures::{stream, StreamExt, TryFutureExt, TryStreamExt};
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateEmbed, Mentionable, ResolvedOption, ResolvedValue, UserId,
};
use zayden_core::SlashCommand;

use crate::{Error, Result};
use crate::utils::{embed_response, message_response};

use super::FamilyRow;

pub struct Partners;

#[async_trait]
impl SlashCommand<Error> for Partners {
    async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
        let user = match interaction.data.options().first() {
            Some(ResolvedOption {
                value: ResolvedValue::User(user, _),
                ..
            }) => *user,
            _ => &interaction.user,
        };

        let row = FamilyRow::safe_get(ctx, user.id).await?;

        if row.partner_ids.is_empty() {
            if user == &interaction.user {
                message_response(ctx, interaction, "You're not currently married").await?;
                return Ok(());
            }

            message_response(
                ctx,
                interaction,
                format!("{} is not currently married", user.mention()),
            )
            .await?;
            return Ok(());
        }

        let partners = stream::iter(row.partner_ids)
            .then(|id| {
                UserId::new(id as u64)
                    .to_user(ctx)
                    .map_ok(|user| user.mention().to_string())
            })
            .try_collect::<Vec<_>>()
            .await?;

        let desc = if user == &interaction.user {
            format!("You are currently married to {}", partners.join(", "))
        } else {
            format!(
                "{} is currently married to {}",
                user.mention(),
                partners.join(", ")
            )
        };

        embed_response(ctx, interaction, CreateEmbed::new().description(desc)).await?;

        Ok(())
    }

    fn register() -> CreateCommand {
        CreateCommand::new("partners")
            .description("List who you are married to.")
            .add_option(CreateCommandOption::new(
                CommandOptionType::User,
                "user",
                "The user to check. Leave blank to check yourself.",
            ))
    }
}
