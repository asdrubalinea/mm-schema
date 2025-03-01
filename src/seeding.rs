use chrono::NaiveDate;

use crate::{
    error::Result,
    models::{AssetType, NormalBalance},
};

use super::Database;

pub(crate) fn init_sample_data(db: &Database) -> Result<()> {
    db.begin_transaction()?;

    init_account_types(db)?;
    init_assets(db)?;

    let opening_date = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();

    init_asset_accounts(db, opening_date)?;
    init_liability_accounts(db, opening_date)?;
    init_equity_accounts(db, opening_date)?;
    init_income_accounts(db, opening_date)?;
    init_expense_accounts(db, opening_date)?;

    db.commit_transaction()?;
    Ok(())
}

const SAMPLE_ACCOUNT_TYPES: [(&str, NormalBalance, Option<&str>); 5] = [
    (
        "Asset",
        NormalBalance::Debit,
        Some("Resources owned by the entity"),
    ),
    (
        "Liability",
        NormalBalance::Credit,
        Some("Debts and obligations"),
    ),
    (
        "Equity",
        NormalBalance::Credit,
        Some("Net worth and capital"),
    ),
    ("Income", NormalBalance::Credit, Some("Revenue and gains")),
    ("Expense", NormalBalance::Debit, Some("Costs and losses")),
];

const SAMPLE_ASSETS: [(&str, &str, AssetType, i64, Option<&str>); 6] = [
    ("USD", "US Dollar", AssetType::Fiat, 2, None),
    ("EUR", "Euro", AssetType::Fiat, 2, None),
    ("AAPL", "Apple Inc.", AssetType::Stock, 8, None),
    ("VWCE", "FTSE All-World", AssetType::Etf, 8, None),
    ("ETH", "Ethereum", AssetType::Crypto, 18, None),
    ("BTC", "Bitcoin", AssetType::Crypto, 18, None),
];

fn init_account_types(db: &Database) -> Result<()> {
    for (name, normal_balance, description) in SAMPLE_ACCOUNT_TYPES {
        db.create_account_type(name, normal_balance, description)?;
    }

    Ok(())
}

fn init_assets(db: &Database) -> Result<()> {
    for (code, name, asset_type, decimals, description) in SAMPLE_ASSETS {
        db.create_asset(code, name, asset_type, decimals, description)?;
    }
    Ok(())
}

fn init_asset_accounts(db: &Database, opening_date: NaiveDate) -> Result<i64> {
    let assets_id = db.create_account("1000", "Assets", 1, None, true, opening_date, None, None)?;

    // Cash and Bank accounts
    let cash_bank_id = db.create_account(
        "1100",
        "Cash and Bank",
        1,
        Some(assets_id),
        true,
        opening_date,
        None,
        None,
    )?;
    for (number, name) in [
        ("1101", "Main Checking Account"),
        ("1102", "Savings Account"),
        ("1103", "Cash Wallet"),
    ] {
        db.create_account(
            number,
            name,
            1,
            Some(cash_bank_id),
            true,
            opening_date,
            None,
            None,
        )?;
    }

    // Investment accounts
    let investment_id = db.create_account(
        "1200",
        "Investment Accounts",
        1,
        Some(assets_id),
        true,
        opening_date,
        None,
        None,
    )?;
    for (number, name) in [
        ("1201", "Stock Brokerage Account"),
        ("1202", "Crypto Wallet"),
    ] {
        db.create_account(
            number,
            name,
            1,
            Some(investment_id),
            true,
            opening_date,
            None,
            None,
        )?;
    }

    // Fixed assets
    let fixed_assets_id = db.create_account(
        "1300",
        "Fixed Assets",
        1,
        Some(assets_id),
        true,
        opening_date,
        None,
        None,
    )?;
    for (number, name) in [("1301", "House"), ("1302", "Vehicle")] {
        db.create_account(
            number,
            name,
            1,
            Some(fixed_assets_id),
            true,
            opening_date,
            None,
            None,
        )?;
    }

    Ok(assets_id)
}

fn init_liability_accounts(db: &Database, opening_date: NaiveDate) -> Result<i64> {
    let liabilities_id = db.create_account(
        "2000",
        "Liabilities",
        2,
        None,
        true,
        opening_date,
        None,
        None,
    )?;

    // Credit cards
    let credit_cards_id = db.create_account(
        "2100",
        "Credit Cards",
        2,
        Some(liabilities_id),
        true,
        opening_date,
        None,
        None,
    )?;
    db.create_account(
        "2101",
        "Main Credit Card",
        2,
        Some(credit_cards_id),
        true,
        opening_date,
        None,
        None,
    )?;

    // Loans
    let loans_id = db.create_account(
        "2200",
        "Loans",
        2,
        Some(liabilities_id),
        true,
        opening_date,
        None,
        None,
    )?;
    for (number, name) in [("2201", "Mortgage"), ("2202", "Car Loan")] {
        db.create_account(
            number,
            name,
            2,
            Some(loans_id),
            true,
            opening_date,
            None,
            None,
        )?;
    }

    Ok(liabilities_id)
}

fn init_equity_accounts(db: &Database, opening_date: NaiveDate) -> Result<i64> {
    let equity_id = db.create_account("3000", "Equity", 3, None, true, opening_date, None, None)?;

    for (number, name) in [("3100", "Opening Balance"), ("3200", "Retained Earnings")] {
        db.create_account(
            number,
            name,
            3,
            Some(equity_id),
            true,
            opening_date,
            None,
            None,
        )?;
    }

    Ok(equity_id)
}

fn init_income_accounts(db: &Database, opening_date: NaiveDate) -> Result<i64> {
    let income_id = db.create_account("4000", "Income", 4, None, true, opening_date, None, None)?;

    db.create_account(
        "4100",
        "Salary",
        4,
        Some(income_id),
        true,
        opening_date,
        None,
        None,
    )?;

    let investment_income_id = db.create_account(
        "4200",
        "Investment Income",
        4,
        Some(income_id),
        true,
        opening_date,
        None,
        None,
    )?;

    for (number, name) in [
        ("4201", "Dividends"),
        ("4202", "Capital Gains"),
        ("4203", "Interest Income"),
    ] {
        db.create_account(
            number,
            name,
            4,
            Some(investment_income_id),
            true,
            opening_date,
            None,
            None,
        )?;
    }

    db.create_account(
        "4300",
        "Other Income",
        4,
        Some(income_id),
        true,
        opening_date,
        None,
        None,
    )?;

    Ok(income_id)
}

fn init_expense_accounts(db: &Database, opening_date: NaiveDate) -> Result<i64> {
    let expenses_id =
        db.create_account("5000", "Expenses", 5, None, true, opening_date, None, None)?;

    // Housing expenses
    let housing_id = db.create_account(
        "5100",
        "Housing",
        5,
        Some(expenses_id),
        true,
        opening_date,
        None,
        None,
    )?;
    for (number, name) in [
        ("5101", "Rent/Mortgage Payment"),
        ("5102", "Utilities"),
        ("5103", "Maintenance"),
    ] {
        db.create_account(
            number,
            name,
            5,
            Some(housing_id),
            true,
            opening_date,
            None,
            None,
        )?;
    }

    // Transportation expenses
    let transport_id = db.create_account(
        "5200",
        "Transportation",
        5,
        Some(expenses_id),
        true,
        opening_date,
        None,
        None,
    )?;
    for (number, name) in [
        ("5201", "Fuel"),
        ("5202", "Car Maintenance"),
        ("5203", "Public Transport"),
    ] {
        db.create_account(
            number,
            name,
            5,
            Some(transport_id),
            true,
            opening_date,
            None,
            None,
        )?;
    }

    // Living expenses
    let living_id = db.create_account(
        "5300",
        "Living",
        5,
        Some(expenses_id),
        true,
        opening_date,
        None,
        None,
    )?;
    for (number, name) in [
        ("5301", "Groceries"),
        ("5302", "Restaurants"),
        ("5303", "Healthcare"),
    ] {
        db.create_account(
            number,
            name,
            5,
            Some(living_id),
            true,
            opening_date,
            None,
            None,
        )?;
    }

    // Entertainment expenses
    let entertainment_id = db.create_account(
        "5400",
        "Entertainment",
        5,
        Some(expenses_id),
        true,
        opening_date,
        None,
        None,
    )?;
    for (number, name) in [("5401", "Streaming Services"), ("5402", "Hobbies")] {
        db.create_account(
            number,
            name,
            5,
            Some(entertainment_id),
            true,
            opening_date,
            None,
            None,
        )?;
    }

    // Financial expenses
    let financial_id = db.create_account(
        "5500",
        "Financial Expenses",
        5,
        Some(expenses_id),
        true,
        opening_date,
        None,
        None,
    )?;
    for (number, name) in [
        ("5501", "Bank Fees"),
        ("5502", "Credit Card Interest"),
        ("5503", "Investment Fees"),
    ] {
        db.create_account(
            number,
            name,
            5,
            Some(financial_id),
            true,
            opening_date,
            None,
            None,
        )?;
    }

    Ok(expenses_id)
}
