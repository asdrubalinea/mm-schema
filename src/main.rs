mod db;
mod error;
mod models;

use db::seeding::init_sample_data;

use crate::db::Database;

fn main() -> error::Result<()> {
    // Example usage
    let db = Database::new("./ciao.db")?;

    db.init_schema()?;
    init_sample_data(&db).unwrap();

    println!("Database initialized with sample data");

    Ok(())
}
