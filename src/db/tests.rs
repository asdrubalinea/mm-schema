use crate::{error::Result, init_sample_data};

use super::*;
use chrono::NaiveDate;

#[test]
fn test_create_account_type() -> Result<()> {
    let db = Database::new_in_memory()?;
    db.init_schema()?;

    let id = db.create_account_type(
        "Test Account Type",
        NormalBalance::Debit,
        Some("Test Description"),
    )?;
    assert!(id > 0);

    Ok(())
}

#[test]
fn test_create_asset() -> Result<()> {
    let db = Database::new_in_memory()?;
    db.init_schema()?;

    let id = db.create_asset(
        "TEST",
        "Test Asset",
        AssetType::Fiat,
        2,
        Some("Test Description"),
    )?;
    assert!(id > 0);

    Ok(())
}

#[test]
fn test_create_account() -> Result<()> {
    let db = Database::new_in_memory()?;
    db.init_schema()?;

    // First create an account type
    let account_type_id = db.create_account_type(
        "Test Account Type",
        NormalBalance::Debit,
        Some("Test Description"),
    )?;

    let opening_date = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();

    let id = db.create_account(
        "1000",
        "Test Account",
        account_type_id,
        None,
        true,
        opening_date,
        None,
        Some("Test Description"),
    )?;
    assert!(id > 0);

    Ok(())
}

#[test]
fn test_init_sample_data() -> Result<()> {
    let db = Database::new_in_memory()?;
    db.init_schema()?;

    init_sample_data(&db)?;

    // Verify some sample data was created
    let mut stmt = db.conn.prepare("SELECT COUNT(*) FROM account_types")?;
    let count: i64 = stmt.query_row([], |row| row.get(0))?;
    assert_eq!(count, 5);

    let mut stmt = db.conn.prepare("SELECT COUNT(*) FROM assets")?;
    let count: i64 = stmt.query_row([], |row| row.get(0))?;
    assert_eq!(count, 4);

    let mut stmt = db.conn.prepare("SELECT COUNT(*) FROM accounts")?;
    let count: i64 = stmt.query_row([], |row| row.get(0))?;
    assert!(count > 0);

    let mut stmt = db.conn.prepare("SELECT COUNT(*) FROM journal_entries")?;
    let count: i64 = stmt.query_row([], |row| row.get(0))?;
    assert_eq!(count, 4);

    let mut stmt = db
        .conn
        .prepare("SELECT COUNT(*) FROM journal_entry_lines")?;
    let count: i64 = stmt.query_row([], |row| row.get(0))?;
    assert_eq!(count, 8);

    Ok(())
}

// #[test]
// fn test_get_general_balance() -> Result<()> {
//     let db = Database::new_in_memory()?;
//     db.init_schema()?;
//     init_sample_data(&db)?;

//     // Create a journal entry and lines to test the balance
//     let date = DateTime::<Utc>::from_naive_utc_and_offset(
//         NaiveDateTime::new(
//             NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
//             chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
//         ),
//         Utc,
//     );

//     // Insert a journal entry
//     db.conn.execute(
//         "INSERT INTO journal_entries (id, date, description, reference_asset_id, status)
//          VALUES (1, ?1, 'Test Entry', 1, 'POSTED')",
//         params![date],
//     )?;

//     // Insert journal entry lines
//     db.conn.execute(
//         "INSERT INTO journal_entry_lines
//          (journal_entry_id, account_id, asset_id, entry_type, amount, reference_amount, line_number)
//          VALUES (1, (SELECT id FROM accounts WHERE account_number = '1101'), 1, 'DEBIT', 1000, 1000, 1)",
//         [],
//     )?;

//     db.conn.execute(
//         "INSERT INTO journal_entry_lines
//          (journal_entry_id, account_id, asset_id, entry_type, amount, reference_amount, line_number)
//          VALUES (1, (SELECT id FROM accounts WHERE account_number = '4100'), 1, 'CREDIT', 1000, 1000, 2)",
//         [],
//     )?;

//     // Test the general balance
//     let balances = db.get_general_balance()?;

//     // We should have at least two rows in the result
//     assert!(balances.len() >= 2);

//     // Find the checking account balance
//     let checking_balance = balances
//         .iter()
//         .find(|row| row.account_number == "1101")
//         .map(|row| row.balance);

//     // Find the salary account balance
//     let salary_balance = balances
//         .iter()
//         .find(|row| row.account_number == "4100")
//         .map(|row| row.balance);

//     // Check that we have the expected balances
//     assert_eq!(checking_balance, Some(1000.0));
//     assert_eq!(salary_balance, Some(1000.0));

//     Ok(())
// }
