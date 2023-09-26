use actix_web::{post, web, App, HttpResponse, HttpServer};
use anyhow::Result;
use history::load_history;
use insert::{insert_results, insert_stats};
use libsql_client::{Client, Config};
use serde::{Deserialize, Serialize};
use sort::{bubble_sort, generate_vec, insertion_sort, merge_sort, quick_sort, selection_sort};
use std::sync::Mutex;
use std::time::{Duration, Instant};

mod history;
mod insert;
mod sort;

pub struct AppState {
    client: Mutex<Client>,
}

#[derive(Serialize)]
pub struct SortResults {
    method: String,
    init_sort: InitSort,
    qty: u32,
    sort_time: Duration,
}

#[derive(Serialize)]
pub struct SortStatsResult {
    method: String,
    init_sort: InitSort,
    qty: u32,
    sort_time: Duration,
    compare: u32,
    swap: u32,
    memory_usage: usize,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum InitSort {
    Sorted,
    Random,
    ReverseSorted,
}

#[derive(Deserialize, Debug)]
struct SortBody {
    qty: u32,
    init_sort: InitSort,
}

#[post("/bubble")]
async fn bubble(data: web::Data<AppState>, req: web::Json<SortBody>) -> HttpResponse {
    let mut vec = generate_vec(&req.init_sort, req.qty as usize);

    let start = Instant::now();
    bubble_sort(&mut vec);
    let end = Instant::now();

    let result = SortResults {
        method: String::from("Bubble sort"),
        init_sort: req.init_sort,
        qty: req.qty,
        sort_time: end - start,
    };

    insert_results(&mut data.client.lock().unwrap(), &result).await;

    return HttpResponse::Ok().json(result);
}

#[post("/insertion")]
async fn insertion(data: web::Data<AppState>, req: web::Json<SortBody>) -> HttpResponse {
    let mut vec = generate_vec(&req.init_sort, req.qty as usize);

    let start = Instant::now();
    let stats = insertion_sort(&mut vec);
    let end = Instant::now();

    let result = SortStatsResult {
        method: String::from("Insertion sort"),
        init_sort: req.init_sort,
        qty: req.qty,
        sort_time: end - start,
        swap: stats.swap,
        compare: stats.compare,
        memory_usage: stats.memory_usage,
    };

    insert_stats(&mut data.client.lock().unwrap(), &result).await;

    return HttpResponse::Ok().json(result);
}

#[post("/merge")]
async fn merge(data: web::Data<AppState>, req: web::Json<SortBody>) -> HttpResponse {
    let mut vec = generate_vec(&req.init_sort, req.qty as usize);

    let start = Instant::now();
    let stats = merge_sort(&mut vec).1;
    let end = Instant::now();

    let result = SortStatsResult {
        method: String::from("Merge sort"),
        init_sort: req.init_sort,
        qty: req.qty,
        sort_time: end - start,
        swap: stats.swap,
        compare: stats.compare,
        memory_usage: stats.memory_usage,
    };

    insert_stats(&mut data.client.lock().unwrap(), &result).await;

    return HttpResponse::Ok().json(result);
}

#[post("/selection")]
async fn selection(data: web::Data<AppState>, req: web::Json<SortBody>) -> HttpResponse {
    let mut vec = generate_vec(&req.init_sort, req.qty as usize);

    let start = Instant::now();
    selection_sort(&mut vec);
    let end = Instant::now();

    let result = SortResults {
        init_sort: req.init_sort,
        method: String::from("Selection sort"),
        qty: req.qty,
        sort_time: end - start,
    };

    insert_results(&mut data.client.lock().unwrap(), &result).await;

    return HttpResponse::Ok().json(result);
}

#[post("/quicksort")]
async fn quicksort(data: web::Data<AppState>, req: web::Json<SortBody>) -> HttpResponse {
    let mut vec = generate_vec(&req.init_sort, req.qty as usize);
    let vec_len = vec.len();

    let start = Instant::now();
    quick_sort(&mut vec, 1, vec_len - 1);
    let end = Instant::now();

    let result = SortResults {
        method: String::from("Quicksort"),
        init_sort: req.init_sort,
        qty: req.qty,
        sort_time: end - start,
    };

    insert_results(&mut data.client.lock().unwrap(), &result).await;

    return HttpResponse::Ok().json(result);
}

#[actix_web::main]
async fn main() -> Result<()> {
    let config = Config::new("file:///tmp/sortstats.db").unwrap();
    let client = Mutex::new(libsql_client::Client::from_config(config).await.unwrap());

    client
        .lock()
        .unwrap()
        .execute(
            "create table if not exists SortResult (
        id integer primary key autoincrement,
        method varchar(50) not null,
        init_sort varchar(50) not null,
        qty int not null,
        sort_time int not null
    )",
        )
        .await
        .unwrap();

    client
        .lock()
        .unwrap()
        .execute(
            "create table if not exists SortStats (
        id integer primary key autoincrement,
        result_id int not null,
        compare int not null,
        swap int not null,
        memory_usage int not null
    )",
        )
        .await
        .unwrap();

    let app_state = web::Data::new(AppState { client });

    HttpServer::new(move || {
        App::new().service(
            web::scope("/sort")
                .app_data(app_state.clone())
                .service(bubble)
                .service(insertion)
                .service(selection)
                .service(merge)
                .service(quicksort)
                .service(load_history),
        )
    })
    .bind(("127.0.0.1", 6969))?
    .run()
    .await?;

    return Ok(());
}
