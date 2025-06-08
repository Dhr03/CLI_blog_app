use std::fs::{self, File};      // read/write files and directories
use std::io::{BufReader, BufWriter};        // I/O Operations
use std::path::Path;            // file path handling

use crate::post::Post;          // Post struct that we defined
use serde_json::Result;         // JSON result type

const FILE_PATH: &str = "data/posts.json";

/// Save a list of posts to a file in JSON format
pub fn save_posts(posts: &[Post]) -> Result<()> {
    // Create parent dir if it doesn't exist
    if let Some(parent) = Path::new(FILE_PATH).parent() {
        fs::create_dir_all(parent).unwrap();
    }
    
    // Create or overwrite a file
    let file = File::create(FILE_PATH).unwrap();
    // Buffered writer (for performance)
    let writer = BufWriter::new(file);
    // JSON pretty writer
    serde_json::to_writer_pretty(writer, posts)
}

/// Load posts from a file. If file doesn't exist, return empty list
pub fn load_posts() -> Result<Vec<Post>> {
    // File doesn't exist
    if !Path::new(FILE_PATH).exists() {
        return Ok(vec![]);
    }

    // Open file and wrap it in buffered reader
    let file = File::open(FILE_PATH).unwrap();
    let reader = BufReader::new(file);
    // Deserialize JSON array into Vec<Post>
    serde_json::from_reader(reader)
}