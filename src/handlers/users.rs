use crate::model::user::User;
use actix_web::web;
use actix_web::{HttpRequest, HttpResponse};
use std::io::Error;

use crate::db::{MySqlPooledConnection, MysqlPool};
use crate::model::user::UserList;

fn mysql_pool_handler(pool: web::Data<MysqlPool>) -> Result<MySqlPooledConnection, HttpResponse> {
    pool.get()
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

// NOTE: test API without db conneciton;
// fn users_mock() -> Result<User, Error> {
//     let _users_json = r#"{"id":"89251ab3-1cdc-4629-9086-ce022cf6669e","first_name":"Marek","last_name":"Majdak","email":"info@sufrago.com","name":"sufrago","create_at":"2019-12-17T17:58:07.533406","avatar_id":"1cb15088-afd4-4d00-a7fc-d95eae1abefb","phone_no":"+48666666666","company_name":"Sufrago sp z o.o.","vat_id":"PL 9512468001"}"#;

//     let user: User = serde_json::from_str(_users_json)?;

//     Ok(user)
// }

pub async fn get(
    _req: HttpRequest,
    pool: web::Data<MysqlPool>,
) -> Result<HttpResponse, HttpResponse> {
    let mysql_pool = mysql_pool_handler(pool)?;
    Ok(HttpResponse::Ok().json(UserList::list(&mysql_pool)))
}

// NOTE: testing connection without DB
// pub async fn get(
//     _req: HttpRequest,
// ) -> Result<HttpResponse, HttpResponse> {
//     let mysql_pool = mysql_pool_handler(pool)?;
//     Ok(HttpResponse::Ok().json(UserList::list(&mysql_pool)))
// }
