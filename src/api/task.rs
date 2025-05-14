

use actix_web::{
    get,
    post,
    put,
    error::ResponseError,
    web::{Path, Json, Data},
    HttpResponse,
    http::{header::ContentType, StatusCode}
};

use serde::{de, Deserialize, Serialize};
use derive_more::Display;
#[derive(Deserialize,Serialize)]
pub struct TaskIdentifier {
    pub task_global_id: String,
}

#[get("/task/{task_global_id}")]
pub async fn get_task() -> Json<String> {
    return Json("hello world".to_string());
}
