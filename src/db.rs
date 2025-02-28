use chrono::NaiveDate;
use rusqlite::{params, Connection, Result};

use crate::models::{AssetType, NormalBalance};

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
        self.conn.execute_batch(include_str!("../schema.sql"))?;
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

    const SAMPLE_ACCOUNT_TYPES: [(
        &'static str,
        NormalBalance,
        Option<&'static str>,
    ); 5] = [
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

    const SAMPLE_ASSETS: [(
        &'static str,
        &'static str,
        AssetType,
        i64,
        Option<&'static str>,
    ); 4] = [
        ("USD", "US Dollar", AssetType::Fiat, 2, None),
        ("EUR", "Euro", AssetType::Fiat, 2, None),
        ("AAPL", "Apple Inc.", AssetType::Stock, 8, None),
        ("ETH", "Ethereum", AssetType::Crypto, 18, None),
    ];

    fn init_account_types(&self) -> Result<()> {
        for (name, normal_balance, description) in Self::SAMPLE_ACCOUNT_TYPES {
            self.create_account_type(name, normal_balance, description)?;
        }
        Ok(())
    }

    fn init_assets(&self) -> Result<()> {
        for (code, name, asset_type, decimals, description) in Self::SAMPLE_ASSETS {
            self.create_asset(code, name, asset_type, decimals, description)?;
        }
        Ok(())
    }

    fn init_asset_accounts(&self, opening_date: NaiveDate) -> Result<i64> {
        let assets_id =
            self.create_account("1000", "Assets", 1, None, true, opening_date, None, None)?;

        // Cash and Bank accounts
        let cash_bank_id = self.create_account(
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
            self.create_account(
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
        let investment_id = self.create_account(
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
            self.create_account(
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
        let fixed_assets_id = self.create_account(
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
            self.create_account(
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

    fn init_liability_accounts(&self, opening_date: NaiveDate) -> Result<i64> {
        let liabilities_id = self.create_account(
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
        let credit_cards_id = self.create_account(
            "2100",
            "Credit Cards",
            2,
            Some(liabilities_id),
            true,
            opening_date,
            None,
            None,
        )?;
        self.create_account(
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
        let loans_id = self.create_account(
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
            self.create_account(
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

    fn init_equity_accounts(&self, opening_date: NaiveDate) -> Result<i64> {
        let equity_id =
            self.create_account("3000", "Equity", 3, None, true, opening_date, None, None)?;

        for (number, name) in [
            ("3100", "Opening Balance"),
            ("3200", "Retained Earnings"),
        ] {
            self.create_account(
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

    fn init_income_accounts(&self, opening_date: NaiveDate) -> Result<i64> {
        let income_id =
            self.create_account("4000", "Income", 4, None, true, opening_date, None, None)?;

        self.create_account(
            "4100",
            "Salary",
            4,
            Some(income_id),
            true,
            opening_date,
            None,
            None,
        )?;

        let investment_income_id = self.create_account(
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
            self.create_account(
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

        self.create_account(
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

    fn init_expense_accounts(&self, opening_date: NaiveDate) -> Result<i64> {
        let expenses_id =
            self.create_account("5000", "Expenses", 5, None, true, opening_date, None, None)?;

        // Housing expenses
        let housing_id = self.create_account(
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
            self.create_account(
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
        let transport_id = self.create_account(
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
            self.create_account(
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
        let living_id = self.create_account(
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
            self.create_account(
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
        let entertainment_id = self.create_account(
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
            self.create_account(
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
        let financial_id = self.create_account(
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
            self.create_account(
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

    pub fn init_sample_data(&self) -> Result<()> {
        self.conn.execute("BEGIN TRANSACTION", [])?;

        self.init_account_types()?;
        self.init_assets()?;

        let opening_date = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
        
        self.init_asset_accounts(opening_date)?;
        self.init_liability_accounts(opening_date)?;
        self.init_equity_accounts(opening_date)?;
        self.init_income_accounts(opening_date)?;
        self.init_expense_accounts(opening_date)?;

        self.conn.execute("COMMIT", [])?;
        Ok(())
    }
}
