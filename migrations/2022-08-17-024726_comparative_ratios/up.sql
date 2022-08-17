-- Your SQL goes here
CREATE TABLE comparative_ratios (
  id SERIAL PRIMARY KEY,
  stock_id INTEGER NOT NULL,
  year INTEGER NOT NULL,
  gross_profit_margin INTEGER,
  operating_profit_margin INTEGER,
  net_profit_margin INTEGER,
  current_asset_return INTEGER,
  tangible_asset_return INTEGER,
  total_liability_return INTEGER,
  revenue_receivable_return INTEGER,
  inventory_receivable_return INTEGER,
  current_asset_liabilities_return INTEGER,
  tangible_asset_total_liabilities_return INTEGER,
  FOREIGN KEY (stock_id) REFERENCES stocks (id)
);
