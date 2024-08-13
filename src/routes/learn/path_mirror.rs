use axum::extract::Path;
use axum::Json;
use crate::model::learn::body_mirror::BodyMirror;

pub async fn path_mirror(Path(id): Path<String>) -> Json<BodyMirror> {
    
    let mirror = BodyMirror {
        message: id
    };
    
    Json(mirror)
}