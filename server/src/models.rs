use diesel::prelude::*;

#[derive(Insertable)]
#[diesel(table_name = crate::schema::records)]
pub struct NewRecord<'a> {
    pub payload: &'a str,
    pub challenge_proof: &'a str,
    pub record_type: &'a str,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::records)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Record {
    pub id: i32,
    pub payload: String,
    pub record_type: String,
}
