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

-- Journal Types
CREATE TABLE journal_types (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE, -- General, Adjustment
    description TEXT
);

-- Journal Entries
CREATE TABLE journal_entries (
    id INTEGER PRIMARY KEY,
    journal_type_id INTEGER NOT NULL,
    date DATETIME NOT NULL,
    description TEXT NOT NULL,
    reference_number TEXT,
    status TEXT CHECK(status IN ('DRAFT', 'POSTED', 'VOID')) DEFAULT 'DRAFT',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    posted_at DATETIME,
    FOREIGN KEY (journal_type_id) REFERENCES journal_types(id)
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

-- Securities
CREATE TABLE securities (
    id INTEGER PRIMARY KEY,
    symbol TEXT NOT NULL,
    name TEXT NOT NULL,
    security_type TEXT NOT NULL, -- stock, bond, mutual fund, etc.
    description TEXT,
    UNIQUE(symbol)
);

-- Account Balances (for performance optimization)
CREATE TABLE account_balances (
    account_id INTEGER NOT NULL,
    security_id INTEGER,
    balance DECIMAL(19,4) NOT NULL,
    quantity DECIMAL(19,4), -- for investment accounts
    last_updated DATETIME NOT NULL,
    FOREIGN KEY (account_id) REFERENCES accounts(id),
    FOREIGN KEY (security_id) REFERENCES securities(id),
    PRIMARY KEY (account_id, security_id)
);

-- Tags for transactions
CREATE TABLE tags (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);

-- Junction table for journal entries and tags
CREATE TABLE journal_entry_tags (
    journal_entry_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    FOREIGN KEY (journal_entry_id) REFERENCES journal_entries(id),
    FOREIGN KEY (tag_id) REFERENCES tags(id),
    PRIMARY KEY (journal_entry_id, tag_id)
);

-- Trigger to ensure balanced entries
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

-- Indexes for better performance
CREATE INDEX idx_journal_entries_date ON journal_entries(date);
CREATE INDEX idx_journal_entry_lines_entry ON journal_entry_lines(journal_entry_id);
CREATE INDEX idx_accounts_number ON accounts(account_number);
