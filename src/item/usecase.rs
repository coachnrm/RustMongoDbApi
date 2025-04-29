use axum::{http::StatusCode, response::IntoResponse, Json};
use bson::oid::ObjectId;
use serde_json::json;

use super::{entity::{InsertItemReq, Item}, repository};

pub async fn insert_one_item(req: InsertItemReq) -> impl IntoResponse {
    match repository::insert_one_item(req).await {
        Ok(r) => (
            StatusCode::CREATED,
            Json(json!({
                "message": format!("Insert item success -> {:?}", r)
            })).into_response()
        ),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "message": e
            })).into_response()
        )
    }
}

pub async fn find_one_item(item_id: ObjectId) -> Result<Item, String> {
    repository::find_one_item(item_id).await 
}


// match repository::find_one_item(item_id).await {
//     Ok(r) => (
//         StatusCode::OK,
//         Json(r).into_response()
//     ),
//     Err(e) => (
//         StatusCode::BAD_REQUEST,
//         Json(json!({
//             "message": e,
//         })).into_response()
//     )
// }