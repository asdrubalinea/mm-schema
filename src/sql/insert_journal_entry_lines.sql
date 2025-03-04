INSERT INTO journal_entry_lines (
    journal_entry_id,
    account_id,
    asset_id,
    entry_type,
    amount,
    description
) VALUES
    (last_insert_rowid(),
     (SELECT id FROM accounts WHERE account_number = :debit_account_number),
     (SELECT id FROM assets WHERE code = :debit_asset_code),
     'DEBIT',
     :amount,
     :description),

    (last_insert_rowid(),
     (SELECT id FROM accounts WHERE account_number = :credit_account_number),
     (SELECT id FROM assets WHERE code = :credit_asset_code),
     'CREDIT',
     :amount,
     :description);
