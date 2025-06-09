use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::fmt;

// Categories of the posts
#[derive(Debug, Serialize, Deserialize)]
pub enum Category {
    Tech, 
    Travel, 
    Cooking, 
    Lifestyle, 
    Uncategorized, 
}

impl FromStr for Category {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_lowercase().as_str() {
            "tech" => Ok(Category::Tech),
            "travel" => Ok(Category::Travel),
            "cooking" => Ok(Category::Cooking), 
            "lifystyle" => Ok(Category::Lifestyle),
            "uncategorized" => Ok(Category::Uncategorized),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Category::Tech => "Tech",
            Category::Travel => "Travel",
            Category::Cooking => "Cooking",
            Category::Lifestyle => "Lifystyle", 
            Category::Uncategorized => "Uncategorized",
        };
        write!(f, "{}", text)
    }
}

// Post struct
#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: u32, 
    pub title: String, 
    pub content: String, 
    pub category: Category, 
    pub user_id: u32, 
}