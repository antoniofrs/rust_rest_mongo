use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct ResultWrapper<T> {
    pub result: T,
}

impl<T> ResultWrapper<T> {
    pub fn of(data: T) -> Self {
        ResultWrapper { result: data }
    }

    pub fn json_of(data: T) -> Json<ResultWrapper<T>> {
        Json(Self::of(data))
    }
}