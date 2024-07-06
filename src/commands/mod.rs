use async_trait::async_trait;
use serenity::all::{CommandInteraction, Context, CreateCommand};
use sqlx::{Database, Pool};

use crate::family_manager::FamilyManager;
use crate::Result;

mod adopt;
mod block;
mod children;
mod marry;
mod parents;

pub use adopt::AdoptCommand;
pub use block::{BlockCommand, UnblockCommand};
pub use children::ChildrenCommand;
pub use marry::MarryCommand;
pub use parents::ParentsCommand;

#[async_trait]
pub trait FamilyCommand<T> {
    async fn run<Db: Database, Manager: FamilyManager<Db>>(
        ctx: &Context,
        interaction: &CommandInteraction,
        pool: &Pool<Db>,
    ) -> Result<T>;

    fn register() -> CreateCommand;
}
