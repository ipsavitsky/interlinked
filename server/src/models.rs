use diesel::{
    AsExpression, FromSqlRow,
    backend::Backend,
    deserialize::FromSql,
    prelude::*,
    serialize::{Output, ToSql},
    sql_types::Text,
    sqlite::Sqlite,
};
use shared::routes::RecordType;
use std::fmt;

/// Newtype wrapper around [`RecordType`] that implements Diesel's [`FromSql`]/[`ToSql`].
///
/// This exists to satisfy the orphan rule — we can't implement traits from `diesel`
/// for a type from `shared`, but we *can* implement them for a local newtype.
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromSqlRow, AsExpression)]
#[diesel(sql_type = Text)]
pub struct DbRecordType(pub RecordType);

impl From<RecordType> for DbRecordType {
    fn from(rt: RecordType) -> Self {
        DbRecordType(rt)
    }
}

impl From<DbRecordType> for RecordType {
    fn from(db: DbRecordType) -> Self {
        db.0
    }
}

impl fmt::Display for DbRecordType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromSql<Text, Sqlite> for DbRecordType {
    fn from_sql(value: <Sqlite as Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        let s = <String as FromSql<Text, Sqlite>>::from_sql(value)?;
        match s.as_str() {
            "link" => Ok(DbRecordType(RecordType::Link)),
            "note" => Ok(DbRecordType(RecordType::Note)),
            other => Err(format!("unknown record type: {other}").into()),
        }
    }
}

impl ToSql<Text, Sqlite> for DbRecordType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> diesel::serialize::Result {
        out.set_value(self.0.to_string());
        Ok(diesel::serialize::IsNull::No)
    }
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::records)]
pub struct NewRecord {
    pub payload: String,
    pub challenge_proof: String,
    pub record_type: DbRecordType,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::records)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Record {
    pub id: i32,
    pub payload: String,
    pub record_type: DbRecordType,
}
