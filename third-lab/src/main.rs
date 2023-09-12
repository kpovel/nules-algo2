use actix_web::{get, web, App, HttpServer, Responder, Result};
use serde::{Deserialize, Serialize};
use sort::{bubble_sort, insertion_sort, merge_sort, selection_sort};
use std::time::{Duration, Instant};

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

#[get("/bubble/{qty}")]
async fn bubble(input: web::Path<InputSort>) -> Result<impl Responder> {
    let mut vec = (0..input.qty).rev().collect::<Vec<u32>>();
    let start = Instant::now();
    bubble_sort(&mut vec);
    let end = Instant::now();

    let results = SortResults {
        method: String::from("Bubble sort"),
        qty: input.qty,
        sort_time: end - start,
    };

    Ok(web::Json(results))
}

#[get("/insertion/{qty}")]
async fn insertion(input: web::Path<InputSort>) -> Result<impl Responder> {
    let mut vec = (0..input.qty).rev().collect::<Vec<u32>>();
    let start = Instant::now();
    insertion_sort(&mut vec);
    let end = Instant::now();

    let results = SortResults {
        method: String::from("Insertion sort"),
        qty: input.qty,
        sort_time: end - start,
    };

    Ok(web::Json(results))
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

#[get("/selection/{qty}")]
async fn selection(input: web::Path<InputSort>) -> Result<impl Responder> {
    let mut vec = (0..input.qty).rev().collect::<Vec<u32>>();
    let start = Instant::now();
    selection_sort(&mut vec);
    let end = Instant::now();

    let results = SortResults {
        method: String::from("Selection sort"),
        qty: input.qty,
        sort_time: end - start,
    };

    Ok(web::Json(results))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/sort")
                .service(bubble)
                .service(insertion)
                .service(selection)
                .service(merge),
        )
    })
    .bind(("127.0.0.1", 6969))?
    .run()
    .await
}
