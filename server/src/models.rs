use diesel::prelude::*;

#[derive(Insertable)]
#[diesel(table_name = crate::schema::records)]
pub struct NewRecord<'a> {
    pub redirect_url: &'a str,
    pub challenge_proof: &'a str,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::records)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Record {
    pub id: i32,
    pub redirect_url: String,
}
