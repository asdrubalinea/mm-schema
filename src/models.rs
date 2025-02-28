use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountType {
    pub id: i64,
    pub name: String,
    pub normal_balance: NormalBalance,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub asset_type: AssetType,
    pub decimals: i64,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeRate {
    pub id: i64,
    pub from_asset_id: i64,
    pub to_asset_id: i64,
    pub rate: Decimal,
    pub date: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: i64,
    pub account_number: String,
    pub name: String,
    pub account_type_id: i64,
    pub parent_account_id: Option<i64>,
    pub is_active: bool,
    pub opening_date: NaiveDate,
    pub closing_date: Option<NaiveDate>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalEntry {
    pub id: i64,
    pub date: DateTime<Utc>,
    pub description: String,
    pub reference_number: Option<String>,
    pub reference_asset_id: i64,
    pub status: EntryStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalEntryLine {
    pub id: i64,
    pub journal_entry_id: i64,
    pub account_id: i64,
    pub asset_id: i64,
    pub entry_type: NormalBalance,
    pub amount: Decimal,
    pub reference_amount: Decimal,
    pub exchange_rate: Option<Decimal>,
    pub line_number: i64,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum NormalBalance {
    Debit,
    Credit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum AssetType {
    Fiat,
    Stock,
    Crypto,
    Commodity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum EntryStatus {
    Draft,
    Posted,
    Void,
}
