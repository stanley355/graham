-- Your SQL goes here
CREATE TABLE per_share_ratios (
  id SERIAL PRIMARY KEY,
  stock_id INTEGER NOT NULL,
  year INTEGER NOT NULL,
  cash_equity BIGINT,
  quick_equity BIGINT,
  current_equity BIGINT,
  tangible_equity BIGINT,
  gross_profit BIGINT,
  operating_profit BIGINT,
  net_profit BIGINT,
  cashflow BIGINT,
  FOREIGN KEY (stock_id) REFERENCES stocks (id)
);
