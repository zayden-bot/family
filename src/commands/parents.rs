use async_trait::async_trait;
// use futures::StreamExt;
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    Mentionable, ResolvedOption, ResolvedValue,
};
use sqlx::{Database, Pool};

use crate::family_manager::FamilyManager;
use crate::{Error, Result};

use super::FamilyCommand;

pub struct ParentsCommand;

#[async_trait]
impl FamilyCommand<()> for ParentsCommand {
    async fn run<Db: Database, Manager: FamilyManager<Db>>(
        ctx: &Context,
        interaction: &CommandInteraction,
        pool: &Pool<Db>,
    ) -> Result<()> {
        // let user = match interaction.data.options().first() {
        //     Some(ResolvedOption {
        //         value: ResolvedValue::User(user, _),
        //         ..
        //     }) => *user,
        //     _ => &interaction.user,
        // };

        // let row = FamilyRow::safe_get(ctx, user.id).await?;

        // if row.parent_ids.is_empty() {
        //     if user == &interaction.user {
        //         message_response(ctx, interaction, "You have no siblings").await?;
        //         return Ok(());
        //     }

        //     message_response(
        //         ctx,
        //         interaction,
        //         format!("{} has no siblings", user.mention()),
        //     )
        //     .await?;
        //     return Ok(());
        // }

        // let pool = PostgresPool::get(ctx).await;
        // let row = FamilyRow::safe_get(ctx, user.id).await?;

        // todo!();

        // let sibling_ids = stream::iter(family_member.siblings().await.into_iter())
        //     .then(|sib_data| async move {
        //         let sib = sib_data.read().await;
        //         sib.id
        //     })
        //     .collect::<Vec<_>>()
        //     .await;

        // let sibling_names = stream::iter(sibling_ids)
        //     .then(|id| async move {
        //         let user_id = UserId::new(id as u64);

        //         if let Some(guild_id) = interaction.guild_id {
        //             if let Ok(member) = guild_id.member(ctx, user_id).await {
        //                 return Ok(member.user.mention().to_string());
        //             }
        //         }

        //         user_id.to_user(ctx).await.map(|user| user.name)
        //     })
        //     .try_collect::<Vec<_>>()
        //     .await?;

        // let siblings_plural = if sibling_names.len() == 1 {
        //     "sibling"
        // } else {
        //     "siblings"
        // };

        // let desc = if user == &interaction.user {
        //     format!(
        //         "You have {} {}:\n{}",
        //         sibling_names.len(),
        //         siblings_plural,
        //         sibling_names.join("\n")
        //     )
        // } else {
        //     format!(
        //         "{} has {} {}:\n{}",
        //         user.mention(),
        //         sibling_names.len(),
        //         siblings_plural,
        //         sibling_names.join("\n")
        //     )
        // };

        // embed_response(ctx, interaction, CreateEmbed::new().description(desc)).await?;

        Ok(())
    }

    fn register() -> CreateCommand {
        CreateCommand::new("parents")
            .description("List who your siblings are.")
            .add_option(CreateCommandOption::new(
                CommandOptionType::User,
                "user",
                "The user to check. Leave blank to check yourself.",
            ))
    }
}
