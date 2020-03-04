use actix_web::web;
use actix_web::{HttpRequest, HttpResponse};

use crate::db::{MySqlPooledConnection, MysqlPool};
use crate::model::user::UserList;

fn mysql_pool_handler(pool: web::Data<MysqlPool>) -> Result<MySqlPooledConnection, HttpResponse> {
    pool.get()
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

pub async fn get(
    _req: HttpRequest,
    pool: web::Data<MysqlPool>,
) -> Result<HttpResponse, HttpResponse> {
    let mysql_pool = mysql_pool_handler(pool)?;
    Ok(HttpResponse::Ok().json(UserList::list(&mysql_pool)))
}
