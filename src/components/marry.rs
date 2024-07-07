use serenity::all::{ComponentInteraction, Context};
use sqlx::{Database, Pool};

use crate::family_manager::FamilyManager;
use crate::{Error, Result};

async fn accept<Db: Database, Manager: FamilyManager<Db>>(
    interaction: &ComponentInteraction,
    pool: &Pool<Db>,
) -> Result<()> {
    let author = &interaction
        .message
        .interaction
        .as_ref()
        .ok_or_else(|| Error::NoInteraction)?
        .user;

    let partner = &interaction.user;

    if !interaction.message.mentions.contains(partner) && partner.id != author.id {
        return Err(Error::UnauthorisedUser);
    };

    let mut row = match Manager::get_row(pool, author.id).await? {
        Some(row) => row,
        None => author.into(),
    };

    let mut partner_row = match Manager::get_row(pool, partner.id).await? {
        Some(row) => row,
        None => partner.into(),
    };

    row.add_partner(&partner_row);
    partner_row.add_partner(&row);

    row.save::<Db, Manager>(pool).await?;
    partner_row.save::<Db, Manager>(pool).await?;

    Ok(())
}

pub async fn decline(ctx: &Context, interaction: &ComponentInteraction) -> Result<()> {
    if !interaction.message.mentions.contains(&interaction.user) {
        return Err(Error::UnauthorisedUser);
    }

    let author = &interaction
        .message
        .interaction
        .as_ref()
        .ok_or_else(|| Error::NoInteraction)?
        .user;

    if author.id == interaction.user.id {
        interaction.delete_response(ctx).await?;
        return Ok(());
    }

    Ok(())
}
