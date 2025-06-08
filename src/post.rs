use serde::{Deserialize, Serialize};

// Categories of the posts
#[derive(Debug, Serialize, Deserialize)]
pub enum Category {
    Tech, 
    Travel, 
    Cooking, 
    Lifestyle, 
    Uncategorized, 
}

// Post struct
#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: u32, 
    pub title: String, 
    pub content: String, 
    pub category: Category, 
    pub author: String, 
}