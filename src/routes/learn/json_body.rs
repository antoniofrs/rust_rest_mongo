use axum::Json;
use crate::model::learn::body_mirror::BodyMirror;

pub async fn body_mirror(Json(body): Json<BodyMirror>) -> Json<BodyMirror> {
    Json(body)
}