use diesel::prelude::*;
use crate::schema::users;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use fake::Fake;
use fake::faker::name::en::{FirstName, LastName};
use fake::faker::internet::en::SafeEmail;


#[derive(Queryable, Selectable, Debug, Serialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    #[diesel(sql_type = diesel::sql_types::Timestamptz)]
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable, Debug, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub email: String,
}

impl NewUser {
    pub fn generate_fake() -> Self {
        NewUser {
            name: format!("{} {}", FirstName().fake::<String>(), LastName().fake::<String>()),
            email: SafeEmail().fake(),
        }
    }
    
    pub fn generate_fake_batch(count: usize) -> Vec<Self> {
        (0..count).map(|_| Self::generate_fake()).collect()
    }
}