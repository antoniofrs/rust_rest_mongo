use crate::model::learn::body_mirror::BodyMirror;
use axum::Json;

pub async fn body_mirror(Json(body): Json<BodyMirror>) -> Json<BodyMirror> {
    Json(body)
}


#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_body_mirror() {
        let input_body = BodyMirror {
            message: "Test".to_owned(),
        };

        let input_json = Json(input_body.clone());

        let response_json = body_mirror(input_json).await;

        assert_eq!(response_json.0, input_body);
    }
}