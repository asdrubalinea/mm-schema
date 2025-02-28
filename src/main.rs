mod models;
mod error;
mod db;

use crate::db::Database;
use crate::models::*;

fn main() -> error::Result<()> {
    // Example usage
    let db = Database::in_memory()?;
    db.init_schema()?;
    
    // Initialize schema and sample data
    db.init_schema()?;
    db.init_sample_data()?;
    println!("Database initialized with sample data");
    
    Ok(())
}
