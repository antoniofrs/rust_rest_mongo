use crate::model::learn::body_mirror::BodyMirror;
use axum::Json;

pub async fn body_mirror(Json(body): Json<BodyMirror>) -> Json<BodyMirror> {
    Json(body)
}