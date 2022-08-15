-- Your SQL goes here
CREATE TABLE per_share_ratios (
  id SERIAL PRIMARY KEY,
  stock_id INTEGER NOT NULL,
  year INTEGER NOT NULL,
  cash_equity BIGINT NOT NULL,
  quick_equity BIGINT NOT NULL,
  current_equity BIGINT NOT NULL,
  tangible_equity BIGINT NOT NULL,
  gross_profit BIGINT NOT NULL,
  operating_profit BIGINT NOT NULL,
  net_profit BIGINT NOT NULL,
  cashflow BIGINT NOT NULL,
  FOREIGN KEY (stock_id) REFERENCES stocks (id)
);
