-- Your SQL goes here
CREATE TABLE income (
  id SERIAL PRIMARY KEY,
  stock_id INTEGER NOT NULL,
  year INTEGER NOT NULL,
  revenue BIGINT NOT NULL,
  gross_profit BIGINT NOT NULL,
  operating_profit BIGINT NOT NULL,
  net_profit BIGINT NOT NULL,
  customer_cashflow BIGINT NOT NULL,
  operating_cashflow BIGINT NOT NULL,
  investing_cashflow BIGINT NOT NULL,
  financing_cashflow BIGINT NOT NULL,
  total_cashflow BIGINT NOT NULL,
  FOREIGN KEY (stock_id) REFERENCES stocks (id)
);
