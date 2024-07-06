use async_trait::async_trait;
use serenity::all::{User, UserId};
use sqlx::{Database, FromRow, Pool};

use crate::relationships::Relationship;
use crate::Result;

#[async_trait]
pub trait FamilyManager<Db: Database> {
    async fn get_row(pool: &Pool<Db>, user_id: impl Into<i64>) -> Result<Option<FamilyRow>>;

    async fn save(pool: &Pool<Db>, row: &FamilyRow) -> Result<()>;
}

#[derive(Debug, Default, Clone, PartialEq, FromRow)]
pub struct FamilyRow {
    pub id: i64,
    pub username: String,
    pub partner_ids: Vec<i64>,
    pub parent_ids: Vec<i64>,
    pub children_ids: Vec<i64>,
    pub blocked_ids: Vec<i64>,
}

impl FamilyRow {
    pub fn add_blocked(&mut self, user_id: UserId) {
        self.blocked_ids.push(user_id.get() as i64);
    }

    pub fn remove_blocked(&mut self, user_id: UserId) {
        self.blocked_ids.retain(|id| *id != user_id.get() as i64);
    }

    pub fn add_child(&mut self, child: &FamilyRow) {
        self.children_ids.push(child.id);
    }

    pub fn add_partner(&mut self, partner: &FamilyRow) {
        self.partner_ids.push(partner.id);
    }

    pub fn add_parent(&mut self, parent: &FamilyRow) {
        self.parent_ids.push(parent.id);
    }

    pub fn relationship(&self, user_id: UserId) -> Relationship {
        let user_id = user_id.get() as i64;

        if self.partner_ids.contains(&user_id) {
            Relationship::Partner
        } else if self.parent_ids.contains(&user_id) {
            Relationship::Parent
        } else if self.children_ids.contains(&user_id) {
            Relationship::Child
        } else {
            Relationship::None
        }
    }

    pub async fn save<Db: Database, Manager: FamilyManager<Db>>(
        &self,
        pool: &Pool<Db>,
    ) -> Result<()> {
        Manager::save(pool, self).await
    }
}

impl From<&User> for FamilyRow {
    fn from(user: &User) -> Self {
        Self {
            id: user.id.get() as i64,
            username: user.name.clone(),
            ..Default::default()
        }
    }
}
