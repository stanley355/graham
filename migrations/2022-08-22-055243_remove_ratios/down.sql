-- Your SQL goes here
CREATE TABLE comparative_ratios (
  id SERIAL PRIMARY KEY,
  stock_id INTEGER NOT NULL,
  year INTEGER NOT NULL,
  gross_profit_margin INTEGER NOT NULL,
  operating_profit_margin INTEGER NOT NULL,
  net_profit_margin INTEGER NOT NULL,
  current_asset_return INTEGER NOT NULL,
  tangible_asset_return INTEGER NOT NULL,
  total_liability_return INTEGER NOT NULL,
  revenue_receivable_return INTEGER NOT NULL,
  revenue_inventory_return INTEGER NOT NULL,
  current_asset_liabilities_return INTEGER NOT NULL,
  tangible_asset_total_liabilities_return INTEGER NOT NULL,
  FOREIGN KEY (stock_id) REFERENCES stocks (id)
);

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

