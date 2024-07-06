use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateEmbed, Mentionable, ResolvedValue,
};
use zayden_core::parse_options;

use crate::sqlx_lib::PostgresPool;
use crate::utils::embed_response;

use super::{FamilyError, FamilyRow, Result};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
    interaction.defer(ctx).await?;

    let options = interaction.data.options();
    let options = parse_options(&options);

    let user = match options.get("user") {
        Some(ResolvedValue::User(user, _)) => *user,
        _ => unreachable!("User option is required and must be a user."),
    };

    let other = match options.get("other") {
        Some(ResolvedValue::User(user, _)) => *user,
        _ => &interaction.user,
    };

    if user == other {
        return Err(FamilyError::SameUser(user.id));
    }

    let pool = PostgresPool::get(ctx).await;

    let user_info = FamilyRow::get(&pool, other.id).await?;

    let relationship = user_info.relationship(user.id)?;

    if other == &interaction.user {
        embed_response(
            ctx,
            interaction,
            CreateEmbed::new().description(format!("{} is your {}", user.mention(), relationship)),
        )
        .await?;
    } else {
        embed_response(
            ctx,
            interaction,
            CreateEmbed::new().description(format!(
                "{} is {}'s {}",
                other.mention(),
                user.mention(),
                relationship
            )),
        )
        .await?;
    }

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("relationship")
        .description("View the relationship between two users.")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::User,
                "user",
                "The user you want to view the relationship of.",
            )
            .required(true),
        )
        .add_option(CreateCommandOption::new(
            CommandOptionType::User,
            "other",
            "The other user you want to view the relationship of.",
        ))
}
