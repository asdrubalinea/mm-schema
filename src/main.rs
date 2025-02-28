mod models;
mod error;
mod db;

use crate::db::Database;
use crate::models::*;

fn main() -> error::Result<()> {
    // Example usage
    let db = Database::in_memory()?;
    db.init_schema()?;
    
    // Create an account type
    let account_type_id = db.create_account_type(
        "Asset",
        NormalBalance::Debit,
        Some("Asset accounts")
    )?;
    
    // Create an asset
    let asset_id = db.create_asset(
        "USD",
        "US Dollar",
        AssetType::Fiat,
        2,
        Some("United States Dollar")
    )?;
    
    println!("Created account type with id: {}", account_type_id);
    println!("Created asset with id: {}", asset_id);
    
    Ok(())
}
