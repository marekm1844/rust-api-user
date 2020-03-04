use crate::schema::user;
use chrono::NaiveDateTime;
use diesel::MysqlConnection;

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub name: String,
    pub create_at: NaiveDateTime,
    pub avatar_id: Option<String>,
    pub phone_no: Option<String>,
    pub company_name: Option<String>,
    pub vat_id: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UserList(pub Vec<User>);

impl UserList {
    pub fn list(connection: &MysqlConnection) -> Self {
        use crate::schema::user::dsl::*;
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;

        let result = user
            .limit(10)
            .load::<User>(connection)
            .expect("Error loading users");

        UserList(result)
    }
}
