#![allow(unused)]
use chrono::{DateTime, NaiveDate, Utc};
use rusqlite::{params, Connection, Result};

use crate::models::{AssetType, EntryStatus, NormalBalance};

#[cfg(test)]
mod tests;

pub(crate) mod seeding;

pub struct Database {
    conn: Connection,
}

/// Represents a row in the general balance report
#[derive(Debug)]
pub struct GeneralBalanceReport {
    pub account_number: String,
    pub account_name: String,
    pub asset: String,
    pub balance: f32,
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
    pub fn create_account_type<S: AsRef<str>>(
        &self,
        name: S,
        normal_balance: NormalBalance,
        description: Option<S>,
    ) -> Result<i64> {
        let mut stmt = self.conn.prepare(
            "INSERT INTO account_types (name, normal_balance, description)
             VALUES (?1, ?2, ?3) RETURNING id",
        )?;

        let id = stmt.query_row(
            params![
                name.as_ref(),
                format!("{:?}", normal_balance).to_uppercase(),
                description.map(|d| d.as_ref().to_string())
            ],
            |row| row.get(0),
        )?;

        Ok(id)
    }

    // Assets
    pub fn create_asset<S: AsRef<str>>(
        &self,
        code: S,
        name: S,
        asset_type: AssetType,
        decimals: i64,
        description: Option<S>,
    ) -> Result<i64> {
        let mut stmt = self.conn.prepare(
            "INSERT INTO assets (code, name, type, decimals, description)
             VALUES (?1, ?2, ?3, ?4, ?5) RETURNING id",
        )?;

        let id = stmt.query_row(
            params![
                code.as_ref(),
                name.as_ref(),
                format!("{:?}", asset_type).to_uppercase(),
                decimals,
                description.map(|d| d.as_ref().to_string())
            ],
            |row| row.get(0),
        )?;

        Ok(id)
    }

    // Account creation
    #[allow(clippy::too_many_arguments)]
    pub fn create_account<S: AsRef<str>>(
        &self,
        account_number: S,
        name: S,
        account_type_id: i64,
        parent_account_id: Option<i64>,
        is_active: bool,
        opening_date: NaiveDate,
        closing_date: Option<NaiveDate>,
        description: Option<S>,
    ) -> Result<i64> {
        let mut stmt = self.conn.prepare(
            "INSERT INTO accounts (
                account_number, name, account_type_id, parent_account_id,
                is_active, opening_date, closing_date, description
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8) RETURNING id",
        )?;

        let id = stmt.query_row(
            params![
                account_number.as_ref(),
                name.as_ref(),
                account_type_id,
                parent_account_id,
                is_active,
                opening_date,
                closing_date,
                description.map(|d| d.as_ref().to_string())
            ],
            |row| row.get(0),
        )?;

        Ok(id)
    }

    pub fn insert_transaction<S: AsRef<str>>(
        &self,
        date: DateTime<Utc>,
        description: S,
        reference_number: S,
        status: EntryStatus,
    ) -> Result<()> {
        todo!()
    }

    // General Balance Report
    pub fn get_general_balance(&self) -> Result<Vec<GeneralBalanceReport>> {
        let mut stmt = self
            .conn
            .prepare(include_str!("../sql/general_balance.sql"))?;
        let rows = stmt.query_map([], |row| {
            Ok(GeneralBalanceReport {
                account_number: row.get(0)?,
                account_name: row.get(1)?,
                asset: row.get(2)?,
                balance: row.get(3)?,
            })
        })?;

        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }

        Ok(results)
    }
}
