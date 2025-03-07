use chrono::{DateTime, NaiveDate, Utc};
use rusqlite::{named_params, params, Connection, Result, Transaction};

use crate::{
    models::{AssetType, EntryStatus, NormalBalance},
    money::Money,
};

pub struct Database(Connection);

/// Represents a row in the general balance report
#[derive(Debug)]
pub struct GeneralBalanceReport {
    pub account_number: String,
    pub account_name: String,
    pub asset: String,
    pub balance: Money,
}

impl Database {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        Ok(Self(conn))
    }

    pub fn new_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        Ok(Self(conn))
    }

    pub fn init_schema(&self) -> Result<()> {
        self.conn().execute_batch(include_str!("sql/schema.sql"))?;
        Ok(())
    }

    pub(super) fn conn(&self) -> &Connection {
        &self.0
    }

    pub(super) fn transaction(&mut self) -> Result<Transaction> {
        self.0.transaction()
    }

    // Account Types
    pub fn create_account_type<S: AsRef<str>>(
        &mut self,
        name: S,
        normal_balance: NormalBalance,
        description: Option<S>,
    ) -> Result<i64> {
        let t = self.transaction()?;

        let id = {
            let mut stmt = t.prepare(
                "INSERT INTO account_types (name, normal_balance, description)
                 VALUES (?1, ?2, ?3) RETURNING id",
            )?;

            stmt.query_row(
                params![
                    name.as_ref(),
                    format!("{:?}", normal_balance).to_uppercase(),
                    description.map(|d| d.as_ref().to_string())
                ],
                |row| row.get(0),
            )?
        };

        t.commit()?;

        Ok(id)
    }

    // Assets
    pub fn create_asset<S: AsRef<str>>(
        &mut self,
        code: S,
        name: S,
        asset_type: AssetType,
        description: Option<S>,
    ) -> Result<i64> {
        let t = self.transaction()?;

        let id = {
            let mut stmt = t.prepare(
                "INSERT INTO assets (code, name, type, description)
                         VALUES (?1, ?2, ?3, ?4) RETURNING id",
            )?;

            stmt.query_row(
                params![
                    code.as_ref(),
                    name.as_ref(),
                    format!("{:?}", asset_type).to_uppercase(),
                    description.map(|d| d.as_ref().to_string())
                ],
                |row| row.get(0),
            )?
        };

        t.commit()?;

        Ok(id)
    }

    // Account creation
    #[allow(clippy::too_many_arguments)]
    pub fn create_account<S: AsRef<str>>(
        &mut self,
        account_number: S,
        name: S,
        account_type_id: i64,
        parent_account_id: Option<i64>,
        is_active: bool,
        opening_date: NaiveDate,
        closing_date: Option<NaiveDate>,
        description: Option<S>,
    ) -> Result<i64> {
        let t = self.transaction()?;

        let id = {
            let mut stmt = t.prepare(
                "INSERT INTO accounts (
                            account_number, name, account_type_id, parent_account_id,
                            is_active, opening_date, closing_date, description
                        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8) RETURNING id",
            )?;

            stmt.query_row(
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
            )?
        };

        t.commit()?;

        Ok(id)
    }

    #[allow(clippy::too_many_arguments)]
    fn insert_transaction<S: AsRef<str>>(
        transaction: &Transaction,

        date: DateTime<Utc>,
        description: S,
        reference_number: S,
        status: EntryStatus,

        debit_account_number: S,
        credit_account_number: S,

        debit_asset_code: S,
        credit_asset_code: S,

        amount: Money,
    ) -> Result<()> {
        transaction.execute(
            include_str!("sql/insert_journal_entry.sql"),
            named_params! {
                ":date": date,
                ":description": description.as_ref(),
                ":reference_number": reference_number.as_ref(),
                ":status": format!("{:?}", status).to_uppercase(),
            },
        )?;

        transaction.execute(
            include_str!("sql/insert_journal_entry_lines.sql"),
            named_params! {
                ":description": description.as_ref(),
                ":amount": amount,
                ":debit_account_number": debit_account_number.as_ref(),
                ":debit_asset_code": debit_asset_code.as_ref(),
                ":credit_account_number": credit_account_number.as_ref(),
                ":credit_asset_code": credit_asset_code.as_ref(),
            },
        )?;

        Ok(())
    }

    // General Balance Report
    pub fn get_general_balance(&self) -> Result<Vec<GeneralBalanceReport>> {
        let mut stmt = self
            .conn()
            .prepare(include_str!("sql/general_balance.sql"))?;

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
