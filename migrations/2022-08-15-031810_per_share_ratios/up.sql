-- Your SQL goes here
CREATE TABLE per_share_ratios (
  id SERIAL PRIMARY KEY,
  stock_id INTEGER NOT NULL,
  year INTEGER NOT NULL,
  balance_id INTEGER,
  income_id INTEGER,
  cash_equity FLOAT,
  quick_equity FLOAT,
  current_equity FLOAT,
  tangible_equity FLOAT,
  gross_profit FLOAT,
  operating_profit FLOAT,
  net_profit FLOAT,
  cashflow FLOAT,
  FOREIGN KEY (stock_id) REFERENCES stocks (id),
  FOREIGN KEY (balance_id) REFERENCES balance (id),
  FOREIGN KEY (income_id) REFERENCES income (id)
);
