use axum::{http::StatusCode, response::IntoResponse, Json};
use bson::oid::ObjectId;
use mongodb::results::{DeleteResult, UpdateResult};
use serde_json::json;

use super::{entity::{InsertItemReq, Item, ItemBson}, repository};

pub async fn find_items() -> Vec<Item> {
    repository::find_items().await
}

pub async fn insert_one_item(req: InsertItemReq) -> Result<ObjectId, String> {
   repository::insert_one_item(req).await
}

pub async fn find_one_item(item_id: ObjectId) -> Result<Item, String> {
    repository::find_one_item(item_id).await 
}

pub async fn update_item(req: ItemBson) -> Result<UpdateResult, String> {
    repository::update_item(req).await
}

pub async fn delete_item(item_id: ObjectId) -> Result<DeleteResult, String> {
    repository::delete_item(item_id).await
}
