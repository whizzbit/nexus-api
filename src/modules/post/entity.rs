use async_graphql::{Enum, SimpleObject};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::modules::user::User;

#[derive(Copy, Clone, Debug, Deserialize, Enum, PartialEq, Eq, Serialize, sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
pub enum Scope {
    Public,
    Private,
}

impl ToString for Scope {
    fn to_string(&self) -> String {
        match &self {
            Self::Public => String::from("public"),
            Self::Private => String::from("private"),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, SimpleObject)]
pub struct Post {
    pub id: Uuid,
    pub content: String,
    pub author: User,
    pub scope: Scope,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
