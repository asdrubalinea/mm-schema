mod error;
mod interface;
mod models;
mod money;
mod seeding;

#[cfg(test)]
mod tests;

use chrono::Utc;
use money::Money;
use rust_decimal_macros::dec;

use crate::interface::Database;

fn main() -> error::Result<()> {
    // Example usage
    let mut db = Database::new("./ciao.db")?;

    db.init_schema()?;
    seeding::init_sample_data(&mut db).unwrap();

    // db.insert_transaction(
    //     Utc::now(),
    //     "Salary",
    //     "SALARY-01",
    //     models::EntryStatus::Posted,
    //     "1101",
    //     "4100",
    //     "EUR",
    //     "EUR",
    //     Money::new(dec!(3000.0)),
    // )?;

    let balance = db.get_general_balance()?;
    dbg!(balance);

    Ok(())
}
