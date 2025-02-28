-- 1. Receiving a salary (3000 USD)
BEGIN TRANSACTION;

INSERT INTO journal_entries (
    date,
    description,
    reference_number,
    reference_asset_id
) VALUES (
    '2023-05-25',
    'Monthly Salary from Evil Corp.',
    'SAL-2023-05',
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
    -- Debit Checking Account (receive money)
    (last_insert_rowid(),
     (SELECT id FROM accounts WHERE account_number = '1101'), -- Main Checking
     (SELECT id FROM assets WHERE code = 'USD'),
     'DEBIT',
     3000.00,
     3000.00,
     1.0,
     1,
     'Salary deposit'),
    -- Credit Salary Income
    (last_insert_rowid(),
     (SELECT id FROM accounts WHERE account_number = '4100'), -- Salary
     (SELECT id FROM assets WHERE code = 'USD'),
     'CREDIT',
     3000.00,
     3000.00,
     1.0,
     2,
     'Monthly salary');

COMMIT;
