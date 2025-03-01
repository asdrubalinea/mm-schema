BEGIN TRANSACTION;

INSERT INTO journal_entries (
    date,
    description,
    reference_number,
    status
) VALUES (
    '2023-05-25',
    'Monthly Salary from Evil Corp.',
    'SAL-2023-05',
    'POSTED'
);

INSERT INTO journal_entry_lines (
    journal_entry_id,
    account_id,
    asset_id,
    entry_type,
    amount,
    description
) VALUES
    (last_insert_rowid(),
     (SELECT id FROM accounts WHERE account_number = '1101'), -- Main Checking
     (SELECT id FROM assets WHERE code = 'USD'),
     'DEBIT',
     3000.00,
     'Salary deposit'),

    (last_insert_rowid(),
     (SELECT id FROM accounts WHERE account_number = '4100'), -- Salary
     (SELECT id FROM assets WHERE code = 'USD'),
     'CREDIT',
     3000.00,
     'Monthly salary');

COMMIT;
