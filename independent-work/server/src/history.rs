use actix_web::{get, web, HttpResponse};

use crate::AppState;

#[get("/history")]
pub async fn load_history(data: web::Data<AppState>) -> HttpResponse {
    let history = data
        .client
        .lock()
        .unwrap()
        .execute(
            "select SortResult.*,
       SS.compare,
       SS.swap,
       SS.memory_usage
from SortResult
         left join main.SortStats SS on SortResult.id = SS.result_id;",
        )
        .await
        .unwrap()
        .rows;

    return HttpResponse::Ok().json(history);
}
