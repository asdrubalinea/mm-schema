mod db;
mod error;
mod models;
mod money;
mod seeding;

#[cfg(test)]
mod tests;

use chrono::Utc;
use money::Money;
use rust_decimal_macros::dec;

use crate::db::Database;

fn main() -> error::Result<()> {
    // Example usage
    let mut db = Database::new("./ciao.db")?;

    db.init_schema()?;
    seeding::init_sample_data(&mut db).unwrap();

    db.insert_transaction(
        Utc::now(),
        "Ciao",
        "IDK",
        models::EntryStatus::Posted,
        0,
        0,
        0,
        0,
        Money::new(dec!(3000.0)),
    )?;

    let balance = db.get_general_balance()?;
    dbg!(balance);

    Ok(())
}
