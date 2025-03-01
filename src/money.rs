use rusqlite::{
    types::{FromSql, FromSqlError, ToSql},
    Result,
};
use rust_decimal::Decimal;

#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::types::{ToSqlOutput, Value};

    #[test]
    fn test_money_from_string() {
        let valid_cases = [
            ("123.45", true),
            ("0.01", true),
            ("-99.99", true),
            ("abc", false),
            ("12.345.67", false),
        ];

        for (input, should_succeed) in valid_cases {
            let result = Money::try_from(input.to_string());
            assert_eq!(result.is_ok(), should_succeed);
        }
    }

    #[test]
    fn test_money_sql_roundtrip() {
        let original = Money(Decimal::new(12345, 2)); // 123.45

        // Test ToSql
        let sql_value = original.to_sql().unwrap();
        dbg!(sql_value);
        match sql_value {
            ToSqlOutput::Owned(value) if matches!(value, Value::Text(s)) => todo!(),
            _ => panic!(),
        }

        // let str_val = match sql_value {
        //     rusqlite::types::ToSqlOutput::Text(s) => s,
        //     _ => panic!("Expected Text output"),
        // };

        // // Test FromSql
        // let value_ref = Value::Text(str_val).into();
        // let roundtrip = Money::column_result(&value_ref).unwrap();

        // assert_eq!(original, roundtrip);
    }
}
