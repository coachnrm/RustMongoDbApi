use bson::oid::ObjectId;

use crate::config::database::dbconnect;

use super::entity::{self, InsertItemReq};
use std::result::Result;

pub async fn insert_one_item(req: InsertItemReq) -> Result<ObjectId, String> {
    let db: ! = match dbconnect()
}