-- Record a $100 grocery expense paid from checking account
BEGIN TRANSACTION;

INSERT INTO journal_entries (
    journal_type_id,
    date,
    description,
    reference_number
) VALUES (
    1, -- General Journal
    '2023-05-20',
    'Grocery shopping',
    'GJ-2023-001'
);

INSERT INTO journal_entry_lines (
    journal_entry_id,
    account_id,
    entry_type,
    amount,
    line_number,
    description
) VALUES
    (last_insert_rowid(), 5000, 'DEBIT', 100.00, 1, 'Grocery expense'),
    (last_insert_rowid(), 1000, 'CREDIT', 100.00, 2, 'Payment from checking');

-- Add tags if desired
INSERT INTO journal_entry_tags (journal_entry_id, tag_id)
VALUES (last_insert_rowid(), 1); -- Assuming tag_id 1 is 'groceries'

COMMIT;
