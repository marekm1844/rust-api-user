table! {
    user (id) {
        id -> Varchar,
        #[sql_name = "firstName"]
        first_name -> Varchar,
        #[sql_name = "lastName"]
        last_name -> Varchar,
        email -> Varchar,
        name -> Varchar,
        #[sql_name = "createAt"]
        create_at -> Timestamp,
        #[sql_name = "avatarId"]
        avatar_id -> Nullable<Varchar>,
        #[sql_name = "phoneNo"]
        phone_no -> Nullable<Varchar>,
        #[sql_name = "companyName"]
        company_name -> Nullable<Varchar>,
        #[sql_name = "vatId"]
        vat_id -> Nullable<Varchar>,
    }
}
