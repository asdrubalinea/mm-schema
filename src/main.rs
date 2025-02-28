mod db;
mod error;
mod models;

use crate::db::Database;

fn main() -> error::Result<()> {
    // Example usage
    let db = Database::new_in_memory()?;

    db.init_schema()?;
    // Seed with sample data
    // db.init_sample_data()?;

    // println!("Database initialized with sample data");

    Ok(())
}
