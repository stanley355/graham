table! {
    balance (id) {
        id -> Int4,
        stock_id -> Int4,
        year -> Int4,
        cash -> Int8,
        receivables -> Int8,
        inventories -> Int8,
        fixed_asset -> Int8,
        quick_asset -> Int8,
        current_asset -> Int8,
        tangible_asset -> Int8,
        st_liabilities -> Int8,
        lt_liabilities -> Int8,
        total_liabilities -> Int8,
        net_cash_asset -> Int8,
        net_quick_asset -> Int8,
        net_current_asset -> Int8,
        net_tangible_asset -> Int8,
        share_outstanding -> Int8,
    }
}

table! {
    income (id) {
        id -> Int4,
        stock_id -> Int4,
        year -> Int4,
        revenue -> Int8,
        gross_profit -> Int8,
        operating_profit -> Int8,
        net_profit -> Int8,
        customer_cashflow -> Int8,
        operating_cashflow -> Int8,
        investing_cashflow -> Int8,
        financing_cashflow -> Int8,
        total_cashflow -> Int8,
    }
}

table! {
    per_share_ratios (id) {
        id -> Int4,
        stock_id -> Int4,
        year -> Int4,
        cash_equity -> Nullable<Int8>,
        quick_equity -> Nullable<Int8>,
        current_equity -> Nullable<Int8>,
        tangible_equity -> Nullable<Int8>,
        gross_profit -> Nullable<Int8>,
        operating_profit -> Nullable<Int8>,
        net_profit -> Nullable<Int8>,
        cashflow -> Nullable<Int8>,
    }
}

table! {
    stocks (id) {
        id -> Int4,
        code -> Varchar,
        name -> Varchar,
    }
}

joinable!(balance -> stocks (stock_id));
joinable!(income -> stocks (stock_id));
joinable!(per_share_ratios -> stocks (stock_id));

allow_tables_to_appear_in_same_query!(
    balance,
    income,
    per_share_ratios,
    stocks,
);
