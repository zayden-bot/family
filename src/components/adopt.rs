use serenity::all::ComponentInteraction;
use sqlx::{Database, Pool};

use crate::family_manager::FamilyManager;
use crate::{Error, Result};

async fn accept<Db: Database, Manager: FamilyManager<Db>>(
    interaction: &ComponentInteraction,
    pool: &Pool<Db>,
) -> Result<()> {
    let parent_user = &interaction
        .message
        .interaction
        .as_ref()
        .ok_or_else(|| Error::NoInteraction)?
        .user;

    let child_user = &interaction.user;

    if !interaction.message.mentions.contains(child_user) && child_user != parent_user {
        return Err(Error::UnauthorisedUser);
    };

    let mut row = match Manager::get_row(pool, parent_user.id).await? {
        Some(row) => row,
        None => parent_user.into(),
    };

    let mut child_row = match Manager::get_row(pool, child_user.id).await? {
        Some(row) => row,
        None => child_user.into(),
    };

    row.add_child(&child_row);
    child_row.add_parent(&row);

    row.save::<Db, Manager>(pool).await?;
    child_row.save::<Db, Manager>(pool).await?;

    Ok(())
}

async fn decline(interaction: &ComponentInteraction) -> Result<()> {
    if !interaction.message.mentions.contains(&interaction.user) {
        return Err(Error::UnauthorisedUser);
    }

    let command_author = &interaction
        .message
        .interaction
        .as_ref()
        .ok_or_else(|| Error::NoInteraction)?
        .user;

    if command_author == &interaction.user {
        return Err(Error::AdoptCancelled);
    }

    Ok(())
}
