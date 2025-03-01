INSERT INTO journal_entries (
    date,
    description,
    reference_number,
    status
) VALUES (
    :date,
    :description,
    :reference_number,
    :status
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
     :amount,
     :description),

    (last_insert_rowid(),
     (SELECT id FROM accounts WHERE account_number = '4100'), -- Salary
     (SELECT id FROM assets WHERE code = 'USD'),
     'CREDIT',
     :amount,
     :description);
