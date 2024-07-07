use async_trait::async_trait;
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    ResolvedValue,
};
use sqlx::{Database, Pool};
use zayden_core::parse_options;

use crate::family_manager::FamilyManager;
use crate::relationships::Relationship;
use crate::{Error, Result};

use super::FamilyCommand;

pub struct RelationshipCommand;

#[async_trait]
impl FamilyCommand<Relationship> for RelationshipCommand {
    async fn run<Db: Database, Manager: FamilyManager<Db>>(
        ctx: &Context,
        interaction: &CommandInteraction,
        pool: &Pool<Db>,
    ) -> Result<Relationship> {
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
            return Err(Error::SameUser(user.id));
        }

        let user_info = match Manager::get_row(pool, user.id).await? {
            Some(row) => row,
            None => user.into(),
        };

        let relationship = user_info.relationship(other.id);

        Ok(relationship)
    }

    fn register() -> CreateCommand {
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
}
