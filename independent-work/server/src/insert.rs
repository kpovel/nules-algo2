use crate::SortStatsResult;
use libsql_client::{Client, Statement};

use super::SortResults;

pub async fn insert_results(dbclient: &mut Client, result: &SortResults) -> i64 {
    dbclient
        .execute(Statement::with_args(
            "insert into SortResult (method, init_sort, qty, sort_time) values (?, ?, ?, ?)",
            &[
                &result.method,
                &format!("{:?}", result.init_sort),
                &result.qty.to_string(),
                &result.sort_time.as_millis().to_string(),
            ],
        ))
        .await
        .unwrap()
        .last_insert_rowid
        .unwrap()
}

pub async fn insert_stats(dbclient: &mut Client, result: &SortStatsResult) {
    let result_id = insert_results(
        dbclient,
        &SortResults {
            method: result.method.clone(),
            init_sort: result.init_sort.clone(),
            qty: result.qty.clone(),
            sort_time: result.sort_time.clone(),
        },
    ).await;

    dbclient
        .execute(Statement::with_args(
            "insert into SortStats (result_id, compare, swap, memory_usage) values (?, ?, ?, ?)",
            &[
                &result_id.to_string(),
                &result.compare.to_string(),
                &result.swap.to_string(),
                &result.memory_usage.to_string(),
            ],
        ))
        .await
        .unwrap();
}
