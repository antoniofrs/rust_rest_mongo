use axum::extract::Query;
use axum::Json;
use crate::model::learn::query_mirror::QueryParams;

pub async fn query_mirror(Query(query): Query<QueryParams>) -> Json<QueryParams> {
    Json(query)
}