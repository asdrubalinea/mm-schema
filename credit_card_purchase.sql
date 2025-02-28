BEGIN TRANSACTION;

INSERT INTO journal_entries (
    date,
    description,
    reference_number,
    reference_asset_id
) VALUES (
    '2023-05-26',
    'Grocery shopping at Whole Foods',
    'CC-2023-05-26',
    (SELECT id FROM assets WHERE code = 'USD')
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
    -- Debit Groceries Expense
    (last_insert_rowid(),
     (SELECT id FROM accounts WHERE account_number = '5301'), -- Groceries
     (SELECT id FROM assets WHERE code = 'USD'),
     'DEBIT',
     150.00,
     150.00,
     1.0,
     1,
     'Weekly groceries'),
    -- Credit Credit Card (creating the debt)
    (last_insert_rowid(),
     (SELECT id FROM accounts WHERE account_number = '2101'), -- Main Credit Card
     (SELECT id FROM assets WHERE code = 'USD'),
     'CREDIT',
     150.00,
     150.00,
     1.0,
     2,
     'Credit card charge');

COMMIT;
