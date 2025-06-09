use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Comment {
    pub id: u32, 
    pub post_id: u32, 
    pub user_id: u32, 
    pub content: String, 
}