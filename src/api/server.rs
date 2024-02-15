use crate::db::database::{create_lp, delete_lp, get_lp_count, init_lp_database, read_lp, update_lp, LiquidityPool};


use actix_web::{
    get,
    post,
    put,
    error::ResponseError,
    web::Path,
    web::Json,
    web::Data,
    HttpResponse,
    http::{header::ContentType, StatusCode}
};
use serde::{Serialize, Deserialize};
use derive_more::{Display};

// #[derive(Deserialize, Serialize)]
// pub struct TaskIdentifier{
    // task_global_id: String,
// }

// #[get("/task/{task_global_id}")]
// pub async fn get_task(task_identifier: Path<TaskIdentifier>, body: Json<Struct>) -> Json<String> {
    // return Json(task_identifier.into_inner().task_global_id);
// }

#[derive(Debug, Display)]
pub enum LiquidityPoolError {
    LiquidityPoolNotFound,
    BadLiquidityPoolRequest
}

impl ResponseError for LiquidityPoolError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            LiquidityPoolError::LiquidityPoolNotFound => StatusCode::NOT_FOUND,
            LiquidityPoolError::BadLiquidityPoolRequest => StatusCode::BAD_REQUEST
        }
    }
}

#[post("/latest_pairs")]
pub async fn get_latest_pairs() -> Result<Json<LiquidityPool>, LiquidityPoolError> {

}