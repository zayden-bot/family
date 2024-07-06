use std::collections::{HashMap, HashSet};
use std::{fmt::Debug, sync::Arc};

use family_manager::FamilyManager;
use futures::{stream, FutureExt, Stream, StreamExt, TryStreamExt};
use serenity::all::{CommandInteraction, Context, CreateCommand, UserId};
use sqlx::{Database, FromRow, Pool};
// use tokio::sync::RwLock;
use zayden_core::SlashCommand;

pub use error::{Error, Result};

mod commands;
mod components;
mod error;
mod family_manager;
mod relationships;

// pub struct FamilyTreeMember {
//     id: i64,
//     username: String,
//     partners: Vec<Arc<RwLock<FamilyTreeMember>>>,
//     parents: Vec<Arc<RwLock<FamilyTreeMember>>>,
//     pub children: Vec<Arc<RwLock<FamilyTreeMember>>>,
// }

// impl FamilyTreeMember {
//     fn add_parent(&mut self, user_id: impl TryInto<i64>) {
//         todo!()
//     }

//     fn remove_parent(&mut self, user_id: impl TryInto<i64>) {
//         todo!()
//     }

//     fn add_partner(&mut self, user_id: impl TryInto<i64>) {
//         todo!()
//     }

//     fn remove_partner(&mut self, user_id: impl TryInto<i64>) {
//         todo!()
//     }

//     pub async fn add_child(&mut self, user_id: impl TryInto<i64>) -> Result<()> {
//         todo!();
//     }

//     fn remove_child(&mut self, user_id: impl TryInto<i64>) {
//         todo!()
//     }

//     fn partners(&self) -> &Vec<Arc<RwLock<FamilyTreeMember>>> {
//         &self.partners
//     }

//     fn children(&self) -> &Vec<Arc<RwLock<FamilyTreeMember>>> {
//         &self.children
//     }

//     fn get_relationship(&self, target_user: FamilyTreeMember) -> Relationship {
//         todo!()
//     }

//     fn span(&self, people: &mut HashSet<FamilyTreeMember>, add_parents: bool) {
//         todo!()
//     }

//     fn get_root(&self) -> FamilyTreeMember {
//         todo!()
//     }
// }

// #[derive(Debug, Default, Clone, PartialEq, FromRow)]
// struct FamilyRow {
//     pub id: i64,
//     pub username: String,
//     pub partner_ids: Vec<i64>,
//     pub parent_ids: Vec<i64>,
//     pub children_ids: Vec<i64>,
//     pub blocked_ids: Vec<i64>,
// }

// impl FamilyRow {
//     fn parents<'a>(&'a self, pool: &'a PgPool) -> impl Stream<Item = Result<Self>> + 'a {
//         stream::iter(self.parent_ids.iter()).then(|id| FamilyRow::get(pool, *id))
//     }

//     fn partners<'a>(&'a self, pool: &'a PgPool) -> impl Stream<Item = Result<Self>> + 'a {
//         stream::iter(self.partner_ids.iter()).then(|id| FamilyRow::get(pool, *id))
//     }

//     fn sibling_ids<'a>(&'a self, pool: &'a PgPool) -> impl Stream<Item = Result<i64>> + 'a {
//         self.parents(pool)
//             .map_ok(|parent| stream::iter(parent.children_ids).map(Ok::<_, Error>))
//             .try_flatten()
//     }

//     fn siblings<'a>(&'a self, pool: &'a PgPool) -> impl Stream<Item = Result<Self>> + 'a {
//         self.sibling_ids(pool)
//             .and_then(|sib| FamilyRow::get(pool, sib))
//     }

//     fn children<'a>(&'a self, pool: &'a PgPool) -> impl Stream<Item = Result<Self>> + 'a {
//         stream::iter(self.children_ids.iter()).then(|id| FamilyRow::get(pool, *id))
//     }

//     pub async fn add_partners(
//         &mut self,
//         pool: &PgPool,
//         target_id: impl TryInto<i64>,
//         target_username: &str,
//     ) -> Result<()> {
//         let target_id: i64 = target_id.try_into().map_err(|_| Error::InvalidUserId)?;

//         let mut transaction = pool.begin().await?;

//         let result = sqlx::query_as!(
//             FamilyRow,
//             "INSERT INTO family (id, username, partner_ids) VALUES ($1, $2, ARRAY[$3]::bigint[]) ON CONFLICT (id) DO UPDATE SET partner_ids = array_append(family.partner_ids, $3) RETURNING *", self.id, self.username, target_id).fetch_one(&mut *transaction).await?;

//         sqlx::query!("INSERT INTO family (id, username, partner_ids) VALUES ($1, $2, ARRAY[$3]::bigint[]) ON CONFLICT (id) DO UPDATE SET partner_ids = array_append(family.partner_ids, $3)", target_id, target_username, self.id).execute(&mut *transaction).await?;

//         transaction.commit().await?;

//         self.partner_ids = result.partner_ids;
//         Ok(())
//     }

//     pub async fn add_child(
//         &mut self,
//         pool: &PgPool,
//         child_id: impl TryInto<i64>,
//         child_name: &str,
//     ) -> Result<()> {
//         let child_id: i64 = child_id.try_into().map_err(|_| Error::InvalidUserId)?;

//         let mut transaction = pool.begin().await?;

//         let result = sqlx::query_as!(
//             FamilyRow,
//             "INSERT INTO family (id, username, children_ids) VALUES ($1, $2, ARRAY[$3]::bigint[]) ON CONFLICT (id) DO UPDATE SET children_ids = array_append(family.children_ids, $3) RETURNING *", self.id, self.username, child_id).fetch_one(&mut *transaction).await?;

//         sqlx::query!("INSERT INTO family (id, username, parent_ids) VALUES ($1, $2, ARRAY[$3]::bigint[]) ON CONFLICT (id) DO UPDATE SET parent_ids = array_append(family.parent_ids, $3)", child_id, child_name, self.id).execute(&mut *transaction).await?;

//         transaction.commit().await?;

//         self.children_ids = result.children_ids;
//         Ok(())
//     }

//     pub async fn add_blocked(&mut self, pool: &PgPool, target_id: impl TryInto<i64>) -> Result<()> {
//         let target_id: i64 = target_id.try_into().map_err(|_| Error::InvalidUserId)?;

//         let result = sqlx::query_as!(FamilyRow,
//             "INSERT INTO family (id, username, blocked_ids) VALUES ($1, $2, ARRAY[$3]::bigint[]) ON CONFLICT (id) DO UPDATE SET blocked_ids = array_append(family.blocked_ids, $3) RETURNING *", self.id, self.username, target_id).fetch_one(pool).await?;

//         self.blocked_ids = result.blocked_ids;
//         Ok(())
//     }

//     pub async fn remove_blocked(
//         &mut self,
//         pool: &PgPool,
//         target_id: impl TryInto<i64>,
//     ) -> Result<()> {
//         let target_id: i64 = target_id.try_into().map_err(|_| Error::InvalidUserId)?;

//         let result = sqlx::query_as!(FamilyRow,
//             "UPDATE family SET blocked_ids = array_remove(family.blocked_ids, $1) WHERE id = $2 RETURNING *",
//             target_id,
//             self.id
//         )
//         .fetch_one(pool)
//         .await?;

//         self.blocked_ids = result.blocked_ids;
//         Ok(())
//     }
// }

// impl FamilyRow {
//     async fn tree(
//         &self,
//         pool: &PgPool,
//         add_parents: bool,
//         add_partners: bool,
//     ) -> Result<HashMap<i32, Vec<FamilyRow>>> {
//         async fn _tree(
//             pool: &PgPool,
//             user: FamilyRow,
//             mut tree: HashMap<i32, Vec<FamilyRow>>,
//             depth: i32,
//             add_parents: bool,
//             add_partners: bool,
//         ) -> Result<HashMap<i32, Vec<FamilyRow>>> {
//             if tree.entry(depth).or_default().contains(&user) {
//                 return Ok(tree);
//             }

//             for child in user.children(pool).try_collect::<Vec<_>>().await? {
//                 tree = Box::pin(_tree(pool, child, tree, depth + 1, false, true)).await?;
//             }

//             if add_partners {
//                 for partner in user.partners(pool).try_collect::<Vec<_>>().await? {
//                     tree = Box::pin(_tree(pool, partner, tree, depth, true, false)).await?;
//                 }
//             }

//             if add_parents {
//                 for parent in user.parents(pool).try_collect::<Vec<_>>().await? {
//                     tree = Box::pin(_tree(pool, parent, tree, depth - 1, true, true)).await?;
//                 }
//             }

//             tree.entry(depth).or_default().push(user);

//             Ok(tree)
//         }

//         let tree = HashMap::new();
//         _tree(pool, self.clone(), tree, 0, add_parents, add_partners).await
//     }

//     pub fn relationship(&self, user_id: impl TryInto<i64>) -> Result<Relationship> {
//         let user_id = user_id.try_into().map_err(|_| Error::InvalidUserId)?;

//         if self.partner_ids.contains(&user_id) {
//             Ok(Relationship::Partner)
//         } else if self.parent_ids.contains(&user_id) {
//             Ok(Relationship::Parent)
//         } else if self.children_ids.contains(&user_id) {
//             Ok(Relationship::Child)
//         } else {
//             Ok(Relationship::None)
//         }
//     }
// }
