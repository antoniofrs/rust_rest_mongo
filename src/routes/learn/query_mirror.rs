use crate::model::learn::query_mirror::QueryParams;
use axum::extract::Query;
use axum::Json;

pub async fn query_mirror(Query(query): Query<QueryParams>) -> Json<QueryParams> {
    Json(query)
}


#[cfg(test)]
mod tests {
    use crate::model::learn::query_mirror::QueryParams;
    use crate::routes::learn::query_mirror::query_mirror;
    use axum::extract::Query;
    use tokio;

    #[tokio::test]
    async fn test_query_mirror() {
        let query_params = QueryParams {
            id: "value1".to_string(),
            message: "value1".to_string(),
        };

        let input_query = Query(query_params.clone());

        let response_json = query_mirror(input_query).await;

        assert_eq!(response_json.0, query_params);
    }
}