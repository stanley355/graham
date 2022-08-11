table! {
    balance (id) {
        id -> Int4,
        stock_id -> Int4,
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
    stocks (id) {
        id -> Int4,
        code -> Varchar,
        name -> Varchar,
    }
}

joinable!(balance -> stocks (stock_id));

allow_tables_to_appear_in_same_query!(
    balance,
    stocks,
);
