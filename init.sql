-- Initialize basic data
BEGIN TRANSACTION;

-- Initialize Account Types
INSERT INTO account_types (id, name, normal_balance, description) VALUES
    (1, 'Asset', 'DEBIT', 'Resources owned by the entity'),
    (2, 'Liability', 'CREDIT', 'Debts and obligations'),
    (3, 'Equity', 'CREDIT', 'Net worth and capital'),
    (4, 'Income', 'CREDIT', 'Revenue and gains'),
    (5, 'Expense', 'DEBIT', 'Costs and losses');

-- Initialize Assets
INSERT INTO assets (code, name, type, decimals) VALUES
    ('USD', 'US Dollar', 'FIAT', 2),
    ('EUR', 'Euro', 'FIAT', 2),
    ('AAPL', 'Apple Inc.', 'STOCK', 8),
    ('ETH', 'Ethereum', 'CRYPTO', 18);

-- Initialize Accounts
INSERT INTO accounts (account_number, name, account_type_id, parent_account_id, is_active, opening_date) VALUES
    -- Asset Accounts (1000-1999)
    ('1000', 'Assets', 1, NULL, true, '2023-01-01'),
        ('1100', 'Cash and Bank', 1, 1, true, '2023-01-01'),
            ('1101', 'Main Checking Account', 1, 2, true, '2023-01-01'),
            ('1102', 'Savings Account', 1, 2, true, '2023-01-01'),
            ('1103', 'Cash Wallet', 1, 2, true, '2023-01-01'),
        ('1200', 'Investment Accounts', 1, 1, true, '2023-01-01'),
            ('1201', 'Stock Brokerage Account', 1, 5, true, '2023-01-01'),
            ('1202', 'Crypto Wallet', 1, 5, true, '2023-01-01'),
        ('1300', 'Fixed Assets', 1, 1, true, '2023-01-01'),
            ('1301', 'House', 1, 7, true, '2023-01-01'),
            ('1302', 'Vehicle', 1, 7, true, '2023-01-01'),

    -- Liability Accounts (2000-2999)
    ('2000', 'Liabilities', 2, NULL, true, '2023-01-01'),
        ('2100', 'Credit Cards', 2, 10, true, '2023-01-01'),
            ('2101', 'Main Credit Card', 2, 11, true, '2023-01-01'),
        ('2200', 'Loans', 2, 10, true, '2023-01-01'),
            ('2201', 'Mortgage', 2, 13, true, '2023-01-01'),
            ('2202', 'Car Loan', 2, 13, true, '2023-01-01'),

    -- Equity Accounts (3000-3999)
    ('3000', 'Equity', 3, NULL, true, '2023-01-01'),
        ('3100', 'Opening Balance', 3, 16, true, '2023-01-01'),
        ('3200', 'Retained Earnings', 3, 16, true, '2023-01-01'),

    -- Income Accounts (4000-4999)
    ('4000', 'Income', 4, NULL, true, '2023-01-01'),
        ('4100', 'Salary', 4, 19, true, '2023-01-01'),
        ('4200', 'Investment Income', 4, 19, true, '2023-01-01'),
            ('4201', 'Dividends', 4, 21, true, '2023-01-01'),
            ('4202', 'Capital Gains', 4, 21, true, '2023-01-01'),
            ('4203', 'Interest Income', 4, 21, true, '2023-01-01'),
        ('4300', 'Other Income', 4, 19, true, '2023-01-01'),

    -- Expense Accounts (5000-5999)
    ('5000', 'Expenses', 5, NULL, true, '2023-01-01'),
        ('5100', 'Housing', 5, 25, true, '2023-01-01'),
            ('5101', 'Rent/Mortgage Payment', 5, 26, true, '2023-01-01'),
            ('5102', 'Utilities', 5, 26, true, '2023-01-01'),
            ('5103', 'Maintenance', 5, 26, true, '2023-01-01'),
        ('5200', 'Transportation', 5, 25, true, '2023-01-01'),
            ('5201', 'Fuel', 5, 30, true, '2023-01-01'),
            ('5202', 'Car Maintenance', 5, 30, true, '2023-01-01'),
            ('5203', 'Public Transport', 5, 30, true, '2023-01-01'),
        ('5300', 'Living', 5, 25, true, '2023-01-01'),
            ('5301', 'Groceries', 5, 34, true, '2023-01-01'),
            ('5302', 'Restaurants', 5, 34, true, '2023-01-01'),
            ('5303', 'Healthcare', 5, 34, true, '2023-01-01'),
        ('5400', 'Entertainment', 5, 25, true, '2023-01-01'),
            ('5401', 'Streaming Services', 5, 38, true, '2023-01-01'),
            ('5402', 'Hobbies', 5, 38, true, '2023-01-01'),
        ('5500', 'Financial Expenses', 5, 25, true, '2023-01-01'),
            ('5501', 'Bank Fees', 5, 41, true, '2023-01-01'),
            ('5502', 'Credit Card Interest', 5, 41, true, '2023-01-01'),
            ('5503', 'Investment Fees', 5, 41, true, '2023-01-01');

COMMIT;
