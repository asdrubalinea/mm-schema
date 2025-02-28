use chrono::NaiveDate;
use rusqlite::{params, Connection, Result};

use crate::models::{AssetType, NormalBalance};

#[cfg(test)]
mod tests;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        Ok(Self { conn })
    }

    pub fn new_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        Ok(Self { conn })
    }

    pub fn init_schema(&self) -> Result<()> {
        self.conn.execute_batch(include_str!("../sql/schema.sql"))?;
        Ok(())
    }

    // Account Types
    pub fn create_account_type(
        &self,
        name: &str,
        normal_balance: NormalBalance,
        description: Option<&str>,
    ) -> Result<i64> {
        let mut stmt = self.conn.prepare(
            "INSERT INTO account_types (name, normal_balance, description)
             VALUES (?1, ?2, ?3) RETURNING id",
        )?;

        let id = stmt.query_row(
            params![
                name,
                format!("{:?}", normal_balance).to_uppercase(),
                description
            ],
            |row| row.get(0),
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
        description: Option<&str>,
    ) -> Result<i64> {
        let mut stmt = self.conn.prepare(
            "INSERT INTO assets (code, name, type, decimals, description)
             VALUES (?1, ?2, ?3, ?4, ?5) RETURNING id",
        )?;

        let id = stmt.query_row(
            params![
                code,
                name,
                format!("{:?}", asset_type).to_uppercase(),
                decimals,
                description
            ],
            |row| row.get(0),
        )?;

        Ok(id)
    }

    // Account creation
    pub fn create_account(
        &self,
        account_number: &str,
        name: &str,
        account_type_id: i64,
        parent_account_id: Option<i64>,
        is_active: bool,
        opening_date: NaiveDate,
        closing_date: Option<NaiveDate>,
        description: Option<&str>,
    ) -> Result<i64> {
        let mut stmt = self.conn.prepare(
            "INSERT INTO accounts (
                account_number, name, account_type_id, parent_account_id,
                is_active, opening_date, closing_date, description
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8) RETURNING id",
        )?;

        let id = stmt.query_row(
            params![
                account_number,
                name,
                account_type_id,
                parent_account_id,
                is_active,
                opening_date,
                closing_date,
                description
            ],
            |row| row.get(0),
        )?;

        Ok(id)
    }
}
