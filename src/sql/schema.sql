-- Account Types
CREATE TABLE account_types (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    normal_balance TEXT CHECK(normal_balance IN ('DEBIT', 'CREDIT')) NOT NULL,
    description TEXT
);
CREATE INDEX idx_account_types_name ON account_types(name);

-- Assets
CREATE TABLE assets (
    id INTEGER PRIMARY KEY,
    code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    type TEXT NOT NULL CHECK(type IN ('FIAT', 'STOCK', 'BOND', 'ETF', 'ETC', 'ETN', 'CRYPTO', 'COMMODITY')),
    decimals INTEGER NOT NULL,
    description TEXT
);
CREATE INDEX idx_assets_code ON assets(code); -- for asset code lookup
CREATE INDEX idx_assets_name ON assets(name); -- for asset name lookup
CREATE INDEX idx_assets_type ON assets(type); -- for asset type lookup

-- Exchange Rates
CREATE TABLE exchange_rates (
    id INTEGER PRIMARY KEY,
    from_asset_id INTEGER NOT NULL,
    to_asset_id INTEGER NOT NULL,
    rate DECIMAL(19,8) NOT NULL,
    date DATETIME NOT NULL,
    FOREIGN KEY (from_asset_id) REFERENCES assets(id),
    FOREIGN KEY (to_asset_id) REFERENCES assets(id),
    UNIQUE(from_asset_id, to_asset_id, date)
);
CREATE INDEX idx_exchange_rates_asset_pair ON exchange_rates(from_asset_id, to_asset_id);
CREATE INDEX idx_exchange_rates_date ON exchange_rates(date);
CREATE INDEX idx_exchange_rates_lookup ON exchange_rates(from_asset_id, to_asset_id, date); -- For rate lookups

-- Accounts
CREATE TABLE accounts (
    id INTEGER PRIMARY KEY,
    account_number TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    account_type_id INTEGER NOT NULL,
    parent_account_id INTEGER,
    is_active BOOLEAN DEFAULT true,
    opening_date DATE NOT NULL,
    closing_date DATE,
    description TEXT,
    FOREIGN KEY (account_type_id) REFERENCES account_types(id),
    FOREIGN KEY (parent_account_id) REFERENCES accounts(id)
);
CREATE INDEX idx_accounts_number ON accounts(account_number);
CREATE INDEX idx_accounts_type ON accounts(account_type_id);
CREATE INDEX idx_accounts_parent ON accounts(parent_account_id);
CREATE INDEX idx_accounts_active ON accounts(is_active);
CREATE INDEX idx_accounts_hierarchy ON accounts(parent_account_id, account_number); -- For tree traversal

-- Journal Entries
CREATE TABLE journal_entries (
    id INTEGER PRIMARY KEY,
    date DATETIME NOT NULL,
    description TEXT NOT NULL,
    reference_number TEXT,
    reference_asset_id INTEGER NOT NULL,
    status TEXT CHECK(status IN ('DRAFT', 'POSTED', 'VOID')) DEFAULT 'DRAFT',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (reference_asset_id) REFERENCES assets(id)
);
CREATE INDEX idx_journal_entries_date ON journal_entries(date);
CREATE INDEX idx_journal_entries_status ON journal_entries(status);
CREATE INDEX idx_journal_entries_reference ON journal_entries(reference_number);
CREATE INDEX idx_journal_entries_created ON journal_entries(created_at);
CREATE INDEX idx_journal_entries_date_status ON journal_entries(date, status); -- Common query combination

-- Journal Entry Lines
CREATE TABLE journal_entry_lines (
    id INTEGER PRIMARY KEY,
    journal_entry_id INTEGER NOT NULL,
    account_id INTEGER NOT NULL,
    asset_id INTEGER NOT NULL,
    entry_type TEXT CHECK(entry_type IN ('DEBIT', 'CREDIT')) NOT NULL,
    amount DECIMAL(19,8) NOT NULL CHECK(amount > 0),
    description TEXT,
    FOREIGN KEY (journal_entry_id) REFERENCES journal_entries(id),
    FOREIGN KEY (account_id) REFERENCES accounts(id),
    FOREIGN KEY (asset_id) REFERENCES assets(id)
);
CREATE INDEX idx_journal_entry_lines_entry ON journal_entry_lines(journal_entry_id);
CREATE INDEX idx_journal_entry_lines_account ON journal_entry_lines(account_id);
CREATE INDEX idx_journal_entry_lines_asset ON journal_entry_lines(asset_id);
CREATE INDEX idx_journal_entry_lines_account_asset ON journal_entry_lines(account_id, asset_id); -- For balance queries
