use rusqlite::{Connection, params};
use crate::{models::*, error::{Result, Error}};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        Ok(Self { conn })
    }

    pub fn in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        Ok(Self { conn })
    }

    pub fn init_schema(&self) -> Result<()> {
        self.conn.execute_batch(include_str!("../schema.sql"))?;
        Ok(())
    }

    // Account Types
    pub fn create_account_type(&self, name: &str, normal_balance: NormalBalance, description: Option<&str>) -> Result<i64> {
        let mut stmt = self.conn.prepare(
            "INSERT INTO account_types (name, normal_balance, description) 
             VALUES (?1, ?2, ?3) RETURNING id"
        )?;
        
        let id = stmt.query_row(
            params![
                name,
                format!("{:?}", normal_balance).to_uppercase(),
                description
            ],
            |row| row.get(0)
        )?;
        
        Ok(id)
    }

    // Assets
    pub fn create_asset(
        &self,
        code: &str,
        name: &str,
        asset_type: AssetType,
        decimals: i64,
        description: Option<&str>
    ) -> Result<i64> {
        let mut stmt = self.conn.prepare(
            "INSERT INTO assets (code, name, type, decimals, description) 
             VALUES (?1, ?2, ?3, ?4, ?5) RETURNING id"
        )?;
        
        let id = stmt.query_row(
            params![
                code,
                name,
                format!("{:?}", asset_type).to_uppercase(),
                decimals,
                description
            ],
            |row| row.get(0)
        )?;
        
        Ok(id)
    }

    // Implement other CRUD operations as needed...
}
