use core::fmt;

use chrono::{DateTime, Utc};
use rusqlite::{
    ToSql,
    types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef},
};

#[derive(Debug, Clone)]
pub struct DateTimeStamp(pub DateTime<Utc>);

impl FromSql for DateTimeStamp {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let s = value.as_str()?;

        let dt = DateTime::parse_from_rfc3339(s).map_err(|_| FromSqlError::InvalidType)?;

        Ok(DateTimeStamp(dt.with_timezone(&Utc)))
    }
}

impl ToSql for DateTimeStamp {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::Owned(self.0.to_rfc3339().into()))
    }
}

impl fmt::Display for DateTimeStamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.to_rfc3339())
    }
}
