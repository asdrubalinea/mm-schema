mod custom;
mod db;
mod error;
mod models;
mod seeding;

#[cfg(test)]
mod tests;

use seeding::init_sample_data;

use crate::db::Database;

fn main() -> error::Result<()> {
    // Example usage
    let db = Database::new("./mm.db")?;

    let balance = db.get_general_balance()?;
    dbg!(balance);

    // db.init_schema()?;
    // init_sample_data(&db).unwrap();

    Ok(())
}
