use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use bson::oid::ObjectId;
use serde_json::json;

use super::{entity::InsertItemReq, usecase};

pub async fn insert_one_item(Json(req): Json<InsertItemReq>) -> impl IntoResponse {
    usecase::insert_one_item(req).await
}

pub async fn find_one_item(Path(item_id): Path<String>) -> impl IntoResponse {
    let item_object_id = match ObjectId::parse_str(item_id) {
        Ok(id) => id,
        Err(_) => (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "message": "Error: Parsing objectid failed"
            })).into_response()
        )
    };
    
    match usecase::find_one_item(item_object_id).await {
            Ok(r) => (
                StatusCode::OK,
                Json(r).into_response()
            ),
            Err(e) => (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "message": e,
                })).into_response()
            )
        }
}