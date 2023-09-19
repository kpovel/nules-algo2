use actix_web::{post, web, App, HttpServer, Responder, Result};
use serde::{Deserialize, Serialize};
use sort::{bubble_sort, generate_vec, insertion_sort, merge_sort, quick_sort, selection_sort};
use std::time::{Duration, Instant};

mod sort;

#[derive(Serialize)]
struct SortResults {
    method: String,
    qty: u32,
    sort_time: Duration,
}

#[derive(Deserialize, Debug)]
pub enum VecType {
    Sorted,
    Random,
    ReverceSorted,
}

#[derive(Deserialize, Debug)]
struct SortBody {
    qty: u32,
    vec_type: VecType,
}

#[post("/bubble")]
async fn bubble(body: web::Json<SortBody>) -> Result<impl Responder> {
    let mut vec = generate_vec(&body.vec_type, body.qty as usize);

    let start = Instant::now();
    bubble_sort(&mut vec);
    let end = Instant::now();

    let result = SortResults {
        method: String::from("Bubble sort"),
        qty: body.qty,
        sort_time: end - start,
    };

    Ok(web::Json(result))
}

#[post("/insertion")]
async fn insertion(body: web::Json<SortBody>) -> Result<impl Responder> {
    let mut vec = generate_vec(&body.vec_type, body.qty as usize);

    let start = Instant::now();
    insertion_sort(&mut vec);
    let end = Instant::now();

    let result = SortResults {
        method: String::from("Insertion sort"),
        qty: body.qty,
        sort_time: end - start,
    };

    Ok(web::Json(result))
}

#[post("/merge")]
async fn merge(body: web::Json<SortBody>) -> Result<impl Responder> {
    let mut vec = generate_vec(&body.vec_type, body.qty as usize);

    let start = Instant::now();
    merge_sort(&mut vec);
    let end = Instant::now();

    let result = SortResults {
        method: String::from("Merge sort"),
        qty: body.qty,
        sort_time: end - start,
    };

    Ok(web::Json(result))
}

#[post("/selection")]
async fn selection(body: web::Json<SortBody>) -> Result<impl Responder> {
    let mut vec = generate_vec(&body.vec_type, body.qty as usize);

    let start = Instant::now();
    selection_sort(&mut vec);
    let end = Instant::now();

    let result = SortResults {
        method: String::from("Selection sort"),
        qty: body.qty,
        sort_time: end - start,
    };

    Ok(web::Json(result))
}

#[post("/quicksort")]
async fn quicksort(body: web::Json<SortBody>) -> Result<impl Responder> {
    let mut vec = generate_vec(&body.vec_type, body.qty as usize);
    let vec_len = vec.len();

    let start = Instant::now();
    quick_sort(&mut vec, 1, vec_len - 1);
    let end = Instant::now();

    let result = SortResults {
        method: String::from("Quicksort"),
        qty: body.qty,
        sort_time: end - start,
    };

    Ok(web::Json(result))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/sort")
                .service(bubble)
                .service(insertion)
                .service(selection)
                .service(merge)
                .service(quicksort),
        )
    })
    .bind(("127.0.0.1", 6969))?
    .run()
    .await
}
