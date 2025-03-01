WITH balances AS (
    SELECT
        a.account_number,
        a.name AS account_name,
        ast.code AS asset,
        at.normal_balance,
        SUM(
            CASE
                WHEN jel.entry_type = 'DEBIT' THEN jel.amount
                ELSE -jel.amount
            END
        ) AS balance
    FROM accounts a
    JOIN account_types at ON at.id = a.account_type_id
    JOIN journal_entry_lines jel ON jel.account_id = a.id
    JOIN assets ast ON ast.id = jel.asset_id
    JOIN journal_entries je ON je.id = jel.journal_entry_id
    WHERE je.status = 'POSTED'
    GROUP BY a.id, ast.id
)
SELECT
    account_number,
    account_name,
    asset,
    CASE
        WHEN normal_balance = 'DEBIT' THEN balance
        ELSE -balance
    END AS balance
FROM balances
WHERE account_number IN ('1101', '2101', '1201', '4100', '5301')
ORDER BY account_number;
