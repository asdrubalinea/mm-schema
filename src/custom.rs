use rusqlite::{
    types::{FromSql, FromSqlError, ToSql},
    Result,
};
use rust_decimal::Decimal;

pub struct Money(Decimal);

impl TryFrom<String> for Money {
    type Error = FromSqlError;

    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        Ok(Money(
            value
                .parse()
                .map_err(|err| FromSqlError::Other(Box::new(err)))?,
        ))
    }
}

impl ToSql for Money {
    fn to_sql(&self) -> Result<rusqlite::types::ToSqlOutput<'_>> {
        let str_val = self.0.to_string();
        Ok(rusqlite::types::ToSqlOutput::from(str_val))
    }
}

impl FromSql for Money {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let str_val = String::column_result(value)?;
        Money::try_from(str_val)
    }
}
