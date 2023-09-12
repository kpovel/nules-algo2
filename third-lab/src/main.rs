use actix_web::{get, web, App, HttpServer, Responder, Result};
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

use sort::merge_sort;

mod sort;

#[derive(Serialize)]
struct SortResults {
    method: String,
    qty: u32,
    sort_time: Duration,
}

#[derive(Deserialize)]
struct InputSort {
    qty: u32,
}

#[get("/merge/{qty}")]
async fn merge(input: web::Path<InputSort>) -> Result<impl Responder> {
    let mut vec = (0..input.qty).rev().collect::<Vec<u32>>();
    let start = Instant::now();
    merge_sort(&mut vec);
    let end = Instant::now();
    let results = SortResults {
        method: String::from("Merge sort"),
        qty: input.qty,
        sort_time: end - start,
    };

    Ok(web::Json(results))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(web::scope("/sort").service(merge)))
        .bind(("127.0.0.1", 6969))?
        .run()
        .await
}
