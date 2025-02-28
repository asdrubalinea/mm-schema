-- Account Types (Asset, Liability, Income, Expense, Equity)
CREATE TABLE account_types (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    normal_balance TEXT CHECK(normal_balance IN ('DEBIT', 'CREDIT')) NOT NULL,
    description TEXT
);

-- Assets (both currencies and securities)
CREATE TABLE assets (
    id INTEGER PRIMARY KEY,
    code TEXT NOT NULL UNIQUE,      -- USD, EUR, AAPL, ETH, etc.
    name TEXT NOT NULL,
    type TEXT NOT NULL CHECK(type IN ('FIAT', 'STOCK', 'BOND', 'ETF', 'ETC', 'ETN', 'CRYPTO', 'COMMODITY')),
    decimals INTEGER NOT NULL,      -- 2 for USD, 8 for BTC, etc.
    description TEXT
);

-- Exchange Rates (historical prices for any asset pair)
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

-- Journal Entries
CREATE TABLE journal_entries (
    id INTEGER PRIMARY KEY,
    date DATETIME NOT NULL,
    description TEXT NOT NULL,
    reference_number TEXT,
    reference_asset_id INTEGER NOT NULL,  -- Reference asset for value calculation
    status TEXT CHECK(status IN ('DRAFT', 'POSTED', 'VOID')) DEFAULT 'DRAFT',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (reference_asset_id) REFERENCES assets(id)
);

-- Journal Entry Lines
CREATE TABLE journal_entry_lines (
    id INTEGER PRIMARY KEY,
    journal_entry_id INTEGER NOT NULL,
    account_id INTEGER NOT NULL,
    asset_id INTEGER NOT NULL,
    entry_type TEXT CHECK(entry_type IN ('DEBIT', 'CREDIT')) NOT NULL,
    amount DECIMAL(19,8) NOT NULL CHECK(amount > 0),
    reference_amount DECIMAL(19,8) NOT NULL,  -- Amount in reference asset
    exchange_rate DECIMAL(19,8),             -- Rate to reference asset
    line_number INTEGER NOT NULL,
    description TEXT,
    FOREIGN KEY (journal_entry_id) REFERENCES journal_entries(id),
    FOREIGN KEY (account_id) REFERENCES accounts(id),
    FOREIGN KEY (asset_id) REFERENCES assets(id)
);
