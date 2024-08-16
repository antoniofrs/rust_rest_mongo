use crate::model::learn::body_mirror::BodyMirror;
use axum::extract::Path;
use axum::Json;

pub async fn path_mirror(Path(id): Path<String>) -> Json<BodyMirror> {
    
    let mirror = BodyMirror {
        message: id
    };
    
    Json(mirror)
}


#[cfg(test)]
mod tests {
    use axum::extract::Path;
    use tokio;
    use crate::model::learn::body_mirror::BodyMirror;
    use crate::routes::learn::path_mirror::path_mirror;

    #[tokio::test]
    async fn test_path_mirror() {

        let path_value = "TestMessage".to_string();

        let response_json = path_mirror(Path(path_value.clone())).await;

        let expected_body = BodyMirror {
            message: path_value,
        };

        assert_eq!(response_json.0, expected_body);
    }
}