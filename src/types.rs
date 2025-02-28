use chrono::NaiveDate;
use rusqlite::{
    types::{ToSql, ValueRef, FromSql},
    Result as SqliteResult,
    Error as SqliteError,
};

impl ToSql for NaiveDate {
    fn to_sql(&self) -> SqliteResult<ValueRef<'_>> {
        self.format("%Y-%m-%d").to_string().to_sql()
    }
}

impl FromSql for NaiveDate {
    fn column_result(value: ValueRef<'_>) -> SqliteResult<Self> {
        let text = String::column_result(value)?;
        NaiveDate::parse_from_str(&text, "%Y-%m-%d")
            .map_err(|_| SqliteError::FromSqlConversionFailure(
                value.data_type(),
                value.data_len(),
                Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Invalid date format",
                )),
            ))
    }
}
