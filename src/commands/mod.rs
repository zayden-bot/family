use async_trait::async_trait;
use serenity::all::{CommandInteraction, Context, CreateCommand};
use sqlx::{Database, Pool};

use crate::family_manager::FamilyManager;
use crate::Result;

mod adopt;
mod block;
mod children;
mod marry;

#[async_trait]
trait FamilyCommand<T> {
    async fn run<Db: Database, Manager: FamilyManager<Db>>(
        ctx: &Context,
        interaction: &CommandInteraction,
        pool: &Pool<Db>,
    ) -> Result<T>;

    fn register() -> CreateCommand;
}
