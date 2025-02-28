-- Account Types (Asset, Liability, Income, Expense, Equity)
CREATE TABLE account_types (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    normal_balance TEXT CHECK(normal_balance IN ('DEBIT', 'CREDIT')) NOT NULL,
    description TEXT
);

-- Chart of Accounts
CREATE TABLE accounts (
    id INTEGER PRIMARY KEY,
    account_number TEXT NOT NULL UNIQUE, -- Standardized account numbering
    name TEXT NOT NULL,
    account_type_id INTEGER NOT NULL,
    parent_account_id INTEGER, -- For hierarchical CoA
    is_active BOOLEAN DEFAULT true,
    currency TEXT NOT NULL,
    opening_date DATE NOT NULL,
    closing_date DATE,
    description TEXT,
    FOREIGN KEY (account_type_id) REFERENCES account_types(id),
    FOREIGN KEY (parent_account_id) REFERENCES accounts(id)
);

-- Fiscal Years
CREATE TABLE fiscal_years (
    id INTEGER PRIMARY KEY,
    start_date DATE NOT NULL,
    end_date DATE NOT NULL,
    is_closed BOOLEAN DEFAULT false,
    UNIQUE(start_date, end_date)
);

-- Fiscal Periods
CREATE TABLE fiscal_periods (
    id INTEGER PRIMARY KEY,
    fiscal_year_id INTEGER NOT NULL,
    start_date DATE NOT NULL,
    end_date DATE NOT NULL,
    period_number INTEGER NOT NULL,
    is_closed BOOLEAN DEFAULT false,
    FOREIGN KEY (fiscal_year_id) REFERENCES fiscal_years(id),
    UNIQUE(fiscal_year_id, period_number)
);

-- Journal Types
CREATE TABLE journal_types (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE, -- General, Adjusting, Closing
    description TEXT
);

-- Journal Entries
CREATE TABLE journal_entries (
    id INTEGER PRIMARY KEY,
    journal_type_id INTEGER NOT NULL,
    fiscal_period_id INTEGER NOT NULL,
    date DATETIME NOT NULL,
    description TEXT NOT NULL,
    reference_number TEXT,
    status TEXT CHECK(status IN ('DRAFT', 'POSTED', 'VOID')) DEFAULT 'DRAFT',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    posted_at DATETIME,
    voided_at DATETIME,
    FOREIGN KEY (journal_type_id) REFERENCES journal_types(id),
    FOREIGN KEY (fiscal_period_id) REFERENCES fiscal_periods(id)
);

-- Journal Entry Lines
CREATE TABLE journal_entry_lines (
    id INTEGER PRIMARY KEY,
    journal_entry_id INTEGER NOT NULL,
    account_id INTEGER NOT NULL,
    security_id INTEGER,
    entry_type TEXT CHECK(entry_type IN ('DEBIT', 'CREDIT')) NOT NULL,
    amount DECIMAL(19,4) NOT NULL CHECK(amount > 0), -- Always positive
    quantity DECIMAL(19,4), -- for investment transactions
    price DECIMAL(19,4), -- for investment transactions
    line_number INTEGER NOT NULL, -- For ordering within entry
    description TEXT,
    FOREIGN KEY (journal_entry_id) REFERENCES journal_entries(id),
    FOREIGN KEY (account_id) REFERENCES accounts(id),
    FOREIGN KEY (security_id) REFERENCES securities(id)
);

-- Trial Balance
CREATE TABLE trial_balances (
    id INTEGER PRIMARY KEY,
    fiscal_period_id INTEGER NOT NULL,
    account_id INTEGER NOT NULL,
    debit_balance DECIMAL(19,4) NOT NULL DEFAULT 0,
    credit_balance DECIMAL(19,4) NOT NULL DEFAULT 0,
    calculated_at DATETIME NOT NULL,
    FOREIGN KEY (fiscal_period_id) REFERENCES fiscal_periods(id),
    FOREIGN KEY (account_id) REFERENCES accounts(id)
);

-- Securities (unchanged from previous)
CREATE TABLE securities (
    id INTEGER PRIMARY KEY,
    symbol TEXT NOT NULL,
    name TEXT NOT NULL,
    security_type TEXT NOT NULL,
    description TEXT,
    UNIQUE(symbol)
);

-- Audit Trail
CREATE TABLE audit_log (
    id INTEGER PRIMARY KEY,
    table_name TEXT NOT NULL,
    record_id INTEGER NOT NULL,
    action TEXT NOT NULL,
    old_values TEXT,
    new_values TEXT,
    user_id INTEGER,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Triggers for maintaining data integrity
CREATE TRIGGER enforce_balanced_entry
AFTER INSERT ON journal_entry_lines
BEGIN
    SELECT CASE
        WHEN (
            SELECT SUM(CASE entry_type
                WHEN 'DEBIT' THEN amount
                WHEN 'CREDIT' THEN -amount
                END)
            FROM journal_entry_lines
            WHERE journal_entry_id = NEW.journal_entry_id
        ) != 0
        THEN RAISE(ABORT, 'Journal entry must balance')
    END;
END;

-- Indexes
CREATE INDEX idx_journal_entries_period ON journal_entries(fiscal_period_id);
CREATE INDEX idx_journal_entries_date ON journal_entries(date);
CREATE INDEX idx_journal_entry_lines_entry ON journal_entry_lines(journal_entry_id);
CREATE INDEX idx_accounts_number ON accounts(account_number);
