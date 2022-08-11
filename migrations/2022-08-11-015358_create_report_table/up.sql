-- Your SQL goes here
CREATE TABLE balance (
  id SERIAL PRIMARY KEY,
  stock_id INTEGER NOT NULL,
  year INTEGER NOT NULL,
  cash BIGINT NOT NULL,
  receivables BIGINT NOT NULL,
  inventories BIGINT NOT NULL,
  fixed_asset BIGINT NOT NULL,
  quick_asset BIGINT NOT NULL,
  current_asset BIGINT NOT NULL,
  tangible_asset BIGINT NOT NULL,
  st_liabilities BIGINT NOT NULL,
  lt_liabilities BIGINT NOT NULL,
  total_liabilities BIGINT NOT NULL,
  net_cash_asset BIGINT NOT NULL,
  net_quick_asset BIGINT NOT NULL,
  net_current_asset BIGINT NOT NULL,
  net_tangible_asset BIGINT NOT NULL,
  share_outstanding BIGINT NOT NULL,
  FOREIGN KEY (stock_id) REFERENCES stocks (id)
);
