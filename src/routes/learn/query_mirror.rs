use crate::model::learn::query_mirror::QueryParams;
use axum::extract::Query;
use axum::Json;

pub async fn query_mirror(Query(query): Query<QueryParams>) -> Json<QueryParams> {
    Json(query)
}