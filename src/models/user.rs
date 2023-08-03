use crate::{db::Pool, handlers::error::Error, schema::users};
use actix_web::web::Data;
use chrono::{DateTime, Local};
use diesel::{Identifiable, Insertable, RunQueryDsl, Queryable};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Debug, Identifiable, Queryable)]
#[diesel(table_name = users)]
pub struct User {
    id: Uuid,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
    username: String,
}

impl User {
    pub fn new<'a>(pool: Data<Pool>, username: &'a str) -> Result<Self, Error> {
        let new_user = NewUser {
            username: String::from(username),
        };
        let conn = &mut pool.get().unwrap();
        match diesel::insert_into(users::table)
            .values(new_user)
            .get_result::<User>(conn)
        {
            Ok(e) => Ok(e),
            Err(e) => Err(Error::from_diesel_error(e)),
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = users)]
struct NewUser {
    username: String,
}
