use rusqlite::types::{FromSql, ToSql};
use rust_decimal::{prelude::ToPrimitive, Decimal};
use serde::{Deserialize, Serialize};

/// In Rust, money is stored in the Decimal type,
/// while in SQLite, it is stored as an integer with eight decimal places
#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct Money(Decimal);

impl From<i128> for Money {
    /// Convert from an i64 to the [`Money`] type.
    /// Please note that the i64 has eight decimal places.
    fn from(value: i128) -> Self {
        // Convert from i64 with 8 decimal places to Decimal
        // Example: 12345678 -> 0.12345678

        Money(Decimal::new(value, 8))
    }
}

impl ToSql for Money {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        // Convert Decimal to i64 with 8 decimal places
        // Example: 0.12345678 -> 12345678
        let scaled = self.0 * Decimal::new(100000000, 0);

        match scaled.to_i128() {
            Some(value) => Ok(rusqlite::types::ToSqlOutput::from(value)),
            None => Err(rusqlite::Error::ToSqlConversionFailure(Box::new(
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Cannot convert decimal to i128",
                ),
            ))),
        }
    }
}

impl FromSql for Money {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let val = i128::column_result(value)?;
        Ok(Money::from(val))
    }
}

#[allow(unused)]
impl Money {
    pub fn new(amount: Decimal) -> Self {
        Money(amount)
    }

    pub fn from_str(s: &str) -> Result<Self, rust_decimal::Error> {
        Ok(Money(s.parse()?))
    }

    pub fn amount(&self) -> Decimal {
        self.0
    }
}

impl std::ops::Add for Money {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Money(self.0 + other.0)
    }
}

impl std::ops::Sub for Money {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Money(self.0 - other.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::{
        types::{ToSqlOutput, Value, ValueRef},
        Connection,
    };
    use rust_decimal_macros::dec;

    #[test]
    fn test_money_from_string() {
        let valid_cases = [
            ("123.45", true),
            ("0.01", true),
            ("-99.99", true),
            ("1234567890.12", true),
            ("0.001", true),
            ("0.000000000000001", true),
            ("abc", false),
            ("12.345.67", false),
            ("", false),
            (".", false),
        ];

        for (input, should_succeed) in valid_cases {
            let result = Money::from_str(input);
            assert_eq!(
                result.is_ok(),
                should_succeed,
                "Failed for input: {}",
                input
            );
        }
    }

    #[test]
    fn test_money_arithmetic() {
        let m1 = Money::new(dec!(100.50));
        let m2 = Money::new(dec!(50.25));

        let sum = m1 + m2;
        assert_eq!(sum.amount(), dec!(150.75));

        let diff = Money::new(dec!(100.00)) - Money::new(dec!(50.00));
        assert_eq!(diff.amount(), dec!(50.00));
    }

    #[test]
    fn test_money_precision() {
        // Test with maximum precision
        let large_amount = Money::from_str("9999999999999999.9999999999999999").unwrap();
        let small_amount = Money::from_str("0.0000000000000001").unwrap();

        let result = large_amount + small_amount;
        assert_eq!(
            result.amount().to_string(),
            "10000000000000000.000000000000"
        );

        // Test rounding behavior
        let m = Money::from_str("1.234").unwrap();
        assert_eq!(m.amount().round_dp(2).to_string(), "1.23");
    }

    #[test]
    fn test_money_sql_roundtrip() {
        let original = Money(Decimal::new(12345, 2)); // 123.45

        // Test ToSql
        let sql_value = original.to_sql().unwrap();
        let value = match sql_value {
            ToSqlOutput::Owned(Value::Integer(i)) => i,
            _ => panic!("Expected Text output"),
        };

        // Test FromSql
        let value = Value::Integer(value);
        let roundtrip = Money::column_result(ValueRef::from(&value)).unwrap();

        assert_eq!(original, roundtrip);
    }

    #[test]
    fn test_money_max_values() {
        // Maximum positive value for i64 is 9,223,372,036,854,775,807
        // With 8 decimal places, this means our maximum decimal is 92,233,720,368.54775807
        let max_money = Money::from_str("92233720368.54775807").unwrap();
        let min_money = Money::from_str("-92233720368.54775807").unwrap();

        let conn = Connection::open_in_memory().unwrap();
        conn.execute(
            "CREATE TABLE test_max_money (id INTEGER PRIMARY KEY, amount INTEGER NOT NULL)",
            [],
        )
        .unwrap();

        // Test inserting maximum values
        conn.execute(
            "INSERT INTO test_max_money (amount) VALUES (?)",
            [&max_money],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO test_max_money (amount) VALUES (?)",
            [&min_money],
        )
        .unwrap();

        // Verify retrieved values
        let mut stmt = conn
            .prepare("SELECT amount FROM test_max_money ORDER BY id")
            .unwrap();
        let money_iter = stmt
            .query_map([], |row| {
                let value: i128 = row.get(0)?;
                Ok(Money::from(value))
            })
            .unwrap();

        let results: Vec<Money> = money_iter.map(|r| r.unwrap()).collect();
        assert_eq!(results[0], max_money);
        assert_eq!(results[1], min_money);

        // Test that exceeding the maximum value fails
        let too_large = Money::from_str("92233720368.54775808").unwrap();
        let result = conn.execute(
            "INSERT INTO test_max_money (amount) VALUES (?)",
            [&too_large],
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_money_db_operations() {
        let conn = Connection::open_in_memory().unwrap();

        // Create a test table
        conn.execute(
            "CREATE TABLE test_money (id INTEGER PRIMARY KEY, amount INTEGER NOT NULL)",
            [],
        )
        .unwrap();

        // Test inserting and retrieving money values
        let test_amounts = vec![
            Money::new(dec!(0.01)),
            Money::new(dec!(1000000.00)),
            Money::new(dec!(-99.99)),
        ];

        for amount in &test_amounts {
            conn.execute("INSERT INTO test_money (amount) VALUES (?)", [amount])
                .unwrap();
        }

        // Verify retrieved values
        let mut stmt = conn
            .prepare("SELECT amount FROM test_money ORDER BY id")
            .unwrap();
        let money_iter = stmt
            .query_map([], |row| {
                let value: i128 = row.get(0)?;
                Ok(Money::from(value))
            })
            .unwrap();

        for (stored, original) in money_iter.zip(test_amounts) {
            assert_eq!(stored.unwrap(), original);
        }
    }
}
