use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::money::Money;

/// Represents a type of account in the accounting system
///
/// Account types define the basic categories of accounts (e.g., Asset, Liability, Equity)
/// and their normal balance behavior (debit or credit).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountType {
    pub id: i64,
    pub name: String,
    pub normal_balance: NormalBalance,
    pub description: Option<String>,
}

/// Represents a financial asset or currency in the system
///
/// Assets can be various types like fiat currencies, stocks, cryptocurrencies,
/// or commodities, each with their own decimal precision requirements.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub asset_type: AssetType,
    pub decimals: i64,
    pub description: Option<String>,
}

/// Represents an exchange rate between two assets at a specific point in time
///
/// Used for currency conversion and asset value calculations in transactions
/// involving multiple currencies or assets.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeRate {
    pub id: i64,
    pub from_asset_id: i64,
    pub to_asset_id: i64,
    pub rate: Decimal,
    pub date: DateTime<Utc>,
}

/// Represents an individual account in the chart of accounts
///
/// Accounts are hierarchical (can have parent accounts) and track financial activity
/// for specific purposes. They can be activated or deactivated over time.
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

/// Represents a journal entry in the accounting system
///
/// A journal entry is a record of a financial transaction that includes
/// multiple line items affecting different accounts. It maintains its status
/// (draft, posted, or void) and reference information.
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

/// Represents a single line item within a journal entry
///
/// Each line specifies an account, asset, amount, and whether it's a debit or credit.
/// It can include exchange rate information for multi-currency transactions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalEntryLine {
    pub id: i64,
    pub journal_entry_id: i64,
    pub account_id: i64,
    pub asset_id: i64,
    pub entry_type: NormalBalance,
    pub amount: Money,
    pub description: Option<String>,
}

/// Represents the normal balance type of an account
///
/// In accounting, accounts naturally maintain either a debit or credit balance.
/// This determines how increases and decreases affect the account balance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum NormalBalance {
    Debit,
    Credit,
}

/// Categorizes different types of assets in the system
///
/// Each asset type may have different handling requirements for
/// valuation, exchange rates, and transaction processing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum AssetType {
    Fiat,
    Stock,
    Bond,
    Etf,
    Etc,
    Etn,
    Crypto,
    Commodity,
}

/// Represents the current status of a journal entry
///
/// - Draft: Entry is still being prepared
/// - Posted: Entry has been finalized and affects account balances
/// - Void: Entry has been invalidated but remains in the system for audit purposes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum EntryStatus {
    Draft,
    Posted,
    Void,
}
