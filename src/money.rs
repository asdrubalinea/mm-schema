use rusqlite::types::{FromSql, ToSql};
use rust_decimal::{prelude::ToPrimitive, Decimal};
use serde::{Deserialize, Serialize};

/// Rust => Decimal type (float backed by an i128)
/// SQLite => 64 bit integer with eight decimal places (fixed precision)
#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct Money(Decimal);

impl ToSql for Money {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        // Convert Decimal to i64 with 8 decimal places
        // Example: 0.12345678 -> 12345678
        let scaled = self.0 * Decimal::new(100000000, 0);

        match scaled.to_i64() {
            Some(value) => Ok(rusqlite::types::ToSqlOutput::from(value)),
            None => Err(rusqlite::Error::ToSqlConversionFailure(Box::new(
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Cannot convert decimal to i64",
                ),
            ))),
        }
    }
}

impl FromSql for Money {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let val = i64::column_result(value)?;
        Ok(Money::from_sqlite_repr(val))
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

    pub fn as_f64(&self) -> f64 {
        self.0.to_f64().unwrap()
    }

    /// Convert from an i64 stored into SQLite to the [`Money`] type.
    /// Please note that the i64 has eight decimal places.
    fn from_sqlite_repr(value: i64) -> Self {
        // Convert from i64 with 8 decimal places to Decimal
        // Example: 12345678 -> 0.12345678
        Money(Decimal::new(value, 8))
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
    use std::str::FromStr;

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
            ("92233720368.54775807", true),
            ("-92233720368.54775807", true),
            ("0.00000000", true),
            ("0.00000001", true),
            ("0.0000000000000001", true),
            // ("999999999999999999.99999999", false), // Too large
            // ("-999999999999999999.99999999", false), // Too small
            // ("12345678901234567890.1234567890", false), // Too many digits
            // ("12345678901234567890.12345678", false),   // Too many digits
            // ("12345678901234567890.12345678901234567890", false), // Too many digits
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
    fn test_money_from_i64() {
        let cases = vec![
            (12345678, "0.12345678"),
            (100000000, "1.00000000"),
            (0, "0.00000000"),
            (-12345678, "-0.12345678"),
            (i64::MAX, "92233720368.54775807"),
            (i64::MIN, "-92233720368.54775808"),
            (0, "0.00000000"),
            (1, "0.00000001"),
            (-1, "-0.00000001"),
        ];

        for (i64_value, expected_str) in cases {
            let expected = Decimal::from_str(expected_str).unwrap();
            let money = Money::from_sqlite_repr(i64_value);
            assert_eq!(money.0, expected, "Failed for i64 value: {}", i64_value);
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
                let value: i64 = row.get(0)?;
                Ok(Money::from_sqlite_repr(value))
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
                let value: i64 = row.get(0)?;
                Ok(Money::from_sqlite_repr(value))
            })
            .unwrap();

        for (stored, original) in money_iter.zip(test_amounts) {
            assert_eq!(stored.unwrap(), original);
        }
    }

    #[test]
    fn test_money_db_arithmetic() {
        let conn = Connection::open_in_memory().unwrap();

        // Create a test table
        conn.execute(
            "CREATE TABLE test_money_arithmetic (id INTEGER PRIMARY KEY, amount INTEGER NOT NULL)",
            [],
        )
        .unwrap();

        // Insert initial money values
        let m1 = Money::new(dec!(100.50));
        let m2 = Money::new(dec!(50.25));
        conn.execute(
            "INSERT INTO test_money_arithmetic (amount) VALUES (?), (?)",
            [&m1, &m2],
        )
        .unwrap();

        // Perform arithmetic operations in the database
        conn.execute(
            "UPDATE test_money_arithmetic SET amount = amount + ? WHERE id = 1",
            [&m2],
        )
        .unwrap();

        conn.execute(
            "UPDATE test_money_arithmetic SET amount = amount - ? WHERE id = 2",
            [&m1],
        )
        .unwrap();

        // Verify retrieved values
        let mut stmt = conn
            .prepare("SELECT amount FROM test_money_arithmetic ORDER BY id")
            .unwrap();
        let money_iter = stmt
            .query_map([], |row| {
                let value: i64 = row.get(0)?;
                Ok(Money::from_sqlite_repr(value))
            })
            .unwrap();

        let results: Vec<Money> = money_iter.map(|r| r.unwrap()).collect();
        assert_eq!(results[0], m1 + m2);
        assert_eq!(results[1], m2 - m1);
    }

    #[test]
    fn test_money_to_sql_conversion_failure() {
        // This would result in an integer that is too big and can´t be represented in an i64
        let too_large = Money(Decimal::from_str("92233720368.54775808").unwrap());
        let result = too_large.to_sql();
        assert!(result.is_err());
    }

    #[test]
    fn test_money_sql_i64_min_and_max() {
        let original = Money(Decimal::new(i64::MAX, 8)); // 92233720368.54775807

        // Test ToSql
        let sql_value = original.to_sql().unwrap();
        let value = match sql_value {
            ToSqlOutput::Owned(Value::Integer(i)) => i,
            _ => panic!("Expected Integer output"),
        };

        // Test FromSql
        let value = Value::Integer(value);
        let roundtrip = Money::column_result(ValueRef::from(&value)).unwrap();

        assert_eq!(original, roundtrip);

        let original = Money(Decimal::new(i64::MIN, 8)); // -92233720368.54775807

        // Test ToSql
        let sql_value = original.to_sql().unwrap();
        let value = match sql_value {
            ToSqlOutput::Owned(Value::Integer(i)) => i,
            _ => panic!("Expected Integer output"),
        };

        // Test FromSql
        let value = Value::Integer(value);
        let roundtrip = Money::column_result(ValueRef::from(&value)).unwrap();

        assert_eq!(original, roundtrip);
    }

    #[test]
    fn test_money_arithmetic_edge_cases() {
        let zero = Money::new(dec!(0.00));
        let m1 = Money::new(dec!(92233720368.54775807));
        let m2 = Money::new(dec!(-92233720368.54775807));

        assert_eq!((m1 + zero).amount(), dec!(92233720368.54775807));
        assert_eq!((zero + m2).amount(), dec!(-92233720368.54775807));
        assert_eq!((m1 - m1).amount(), dec!(0.00));
        assert_eq!((m2 - m2).amount(), dec!(0.00));
        assert_eq!((m1 + m2).amount(), dec!(0.00));
    }
}
