use rusqlite::{
    types::{FromSql, ToSql},
    Result,
};
use rust_decimal::Decimal;

/// In Rust, money is stored in the Decimal type,
/// while in SQLite, it is stored as an integer with eight decimal places
#[derive(Debug, PartialEq)]
pub struct Money(Decimal);

impl From<i64> for Money {
    /// Convert from an i64 to the [`Money`] type.
    /// Please note that the i64 has eight decimal places.
    fn from(value: i64) -> Self {
        // Convert from i64 with 8 decimal places to Decimal
        // Example: 12345678 -> 0.12345678
        Money(Decimal::new(value, 8))
    }
}

impl ToSql for Money {
    fn to_sql(&self) -> Result<rusqlite::types::ToSqlOutput<'_>> {
        // Convert Decimal to i64 with 8 decimal places
        // Example: 0.12345678 -> 12345678
        let scaled = self.0 * Decimal::new(100000000, 0);
        let value = scaled.to_i64().ok_or(rusqlite::Error::ToSqlConversionFailure)?;
        Ok(rusqlite::types::ToSqlOutput::from(value))
    }
}

impl FromSql for Money {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let val = i64::column_result(value)?;
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
            ToSqlOutput::Owned(Value::Text(s)) => s,
            _ => panic!("Expected Text output"),
        };

        // Test FromSql
        let value = Value::Text(value);
        let roundtrip = Money::column_result(ValueRef::from(&value)).unwrap();

        assert_eq!(original, roundtrip);
    }

    #[test]
    fn test_money_db_operations() {
        let conn = Connection::open_in_memory().unwrap();

        // Create a test table
        conn.execute(
            "CREATE TABLE test_money (id INTEGER PRIMARY KEY, amount TEXT NOT NULL)",
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
                let value: String = row.get(0)?;
                Ok(Money::try_from(value))
            })
            .unwrap();

        for (stored, original) in money_iter.zip(test_amounts) {
            assert_eq!(stored.unwrap().unwrap(), original);
        }
    }
}
