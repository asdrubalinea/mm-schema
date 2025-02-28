-- 3. Paying Credit Card Balance ($150)
BEGIN TRANSACTION;

INSERT INTO journal_entries (
    date,
    description,
    reference_number,
    status,
    reference_asset_id
) VALUES (
    '2023-06-15',
    'Credit Card Payment',
    'CCP-2023-06',
    'POSTED',
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
    -- Debit Credit Card (reducing the debt)
    (last_insert_rowid(),
     (SELECT id FROM accounts WHERE account_number = '2101'), -- Main Credit Card
     (SELECT id FROM assets WHERE code = 'USD'),
     'DEBIT',
     150.00,
     150.00,
     1.0,
     1,
     'Payment to credit card'),
    -- Credit Checking Account (money leaves the bank)
    (last_insert_rowid(),
     (SELECT id FROM accounts WHERE account_number = '1101'), -- Main Checking
     (SELECT id FROM assets WHERE code = 'USD'),
     'CREDIT',
     150.00,
     150.00,
     1.0,
     2,
     'Payment from checking account');

COMMIT;
