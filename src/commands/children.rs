use async_trait::async_trait;
use futures::{stream, StreamExt, TryStreamExt};
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    Mentionable, ResolvedOption, ResolvedValue, UserId,
};
use sqlx::{Database, Pool};

use crate::family_manager::FamilyManager;
use crate::{Error, Result};

use super::FamilyCommand;

pub struct ChildrenCommand;

#[async_trait]
impl FamilyCommand<Vec<String>> for ChildrenCommand {
    async fn run<Db: Database, Manager: FamilyManager<Db>>(
        ctx: &Context,
        interaction: &CommandInteraction,
        pool: &Pool<Db>,
    ) -> Result<Vec<String>> {
        let user = match interaction.data.options().first() {
            Some(ResolvedOption {
                value: ResolvedValue::User(user, _),
                ..
            }) => *user,
            _ => &interaction.user,
        };

        let row = match Manager::get_row(pool, user.id).await? {
            Some(row) => row,
            None => (&interaction.user).into(),
        };

        if row.children_ids.is_empty() {
            if user == &interaction.user {
                return Err(Error::SelfNoChildren);
            }

            return Err(Error::NoChildren(user.id));
        }

        let children: Vec<String> = stream::iter(row.children_ids)
            .then(|id| async move {
                let user_id = UserId::new(id as u64);
                let user = user_id.to_user(ctx).await?;

                Ok::<String, serenity::Error>(user.mention().to_string())
            })
            .try_collect()
            .await?;

        Ok(children)
    }

    fn register() -> CreateCommand {
        CreateCommand::new("children")
            .description("List who your children are.")
            .add_option(CreateCommandOption::new(
                CommandOptionType::User,
                "user",
                "The user to check. Leave blank to check yourself.",
            ))
    }
}
