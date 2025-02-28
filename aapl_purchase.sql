-- 4. Purchasing 10 shares of AAPL at $175 per share
BEGIN TRANSACTION;

-- First, ensure we have the current exchange rate
INSERT INTO exchange_rates (
    from_asset_id,
    to_asset_id,
    rate,
    date
) VALUES (
    (SELECT id FROM assets WHERE code = 'AAPL'),
    (SELECT id FROM assets WHERE code = 'USD'),
    175.00,
    '2023-06-15'
);

INSERT INTO journal_entries (
    date,
    description,
    reference_number,
    reference_asset_id
) VALUES (
    '2023-06-15',
    'Purchase AAPL shares',
    'INV-2023-06-15',
    (SELECT id FROM assets WHERE code = 'USD')  -- Using USD as reference currency
);

INSERT INTO journal_entry_lines (
    journal_entry_id,
    account_id,
    asset_id,
    entry_type,
    amount,
    reference_amount,
    exchange_rate,
    line_number,
    description
) VALUES
    -- Debit Investment Account (receive AAPL shares)
    (last_insert_rowid(),
     (SELECT id FROM accounts WHERE account_number = '1201'), -- Stock Brokerage
     (SELECT id FROM assets WHERE code = 'AAPL'),
     'DEBIT',
     10.00,              -- 10 shares
     1750.00,            -- Value in USD (10 * $175)
     175.00,             -- Price per share
     1,
     'Buy 10 AAPL shares'),
    -- Credit Checking Account (pay with USD)
    (last_insert_rowid(),
     (SELECT id FROM accounts WHERE account_number = '1101'), -- Main Checking
     (SELECT id FROM assets WHERE code = 'USD'),
     'CREDIT',
     1750.00,            -- Total USD paid
     1750.00,            -- Same in reference currency
     1.0,                -- USD/USD rate
     2,
     'Payment for AAPL shares');

COMMIT;
