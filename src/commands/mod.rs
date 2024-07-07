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
mod partners;
mod relationship;
mod siblings;
mod tree;

pub use adopt::Adopt;
pub use block::{Block, Unblock};
pub use children::Children;
pub use marry::Marry;
pub use parents::Parents;
pub use partners::Partners;
pub use relationship::RelationshipCmd;
pub use siblings::Siblings;
pub use tree::Tree;

#[async_trait]
pub trait FamilyCommand<T> {
    async fn run<Db: Database, Manager: FamilyManager<Db>>(
        ctx: &Context,
        interaction: &CommandInteraction,
        pool: &Pool<Db>,
    ) -> Result<T>;

    fn register() -> CreateCommand;
}
