use chrono::Utc;
use diesel::prelude::*;
use diesel::result::Error;

use crate::domain::models::user::{NewUser, User};
use crate::schema::user::dsl::*;

pub fn create_user(conn: &mut PgConnection, new_nickname: &str) -> Result<User, Error> {
    let new_user = NewUser {
        nickname: new_nickname,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(user).values(&new_user).get_result(conn)
}

pub fn get_user(conn: &mut PgConnection, user_id: i32) -> Result<User, Error> {
    user.find(user_id).get_result(conn)
}

pub fn update_user(conn: &mut PgConnection, user_id: i32, new_nickname: &str) -> Result<User, Error> {
    diesel::update(user.find(user_id)).set(nickname.eq(Some(new_nickname.to_string()))).get_result(conn)
}

pub fn delete_user(conn: &mut PgConnection, user_id: i32) -> Result<usize, Error> {
    diesel::delete(user.find(user_id)).execute(conn)
}
