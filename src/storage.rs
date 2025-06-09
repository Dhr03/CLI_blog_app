use std::fs::{self, File};
use std::io::{BufReader, BufWriter};
use std::path::Path;
use crate::post::Post;
use crate::user::User;
use serde_json::Result;

const POSTS_FILE: &str = "data/posts.json";
const USERS_FILE: &str = "data/users.json";

pub fn save_posts(posts: &[Post]) -> Result<()> {
    if let Some(parent) = Path::new(POSTS_FILE).parent() {
        fs::create_dir_all(parent).unwrap();
    }
    let file = File::create(POSTS_FILE).unwrap();
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, posts)
}

pub fn load_posts() -> Result<Vec<Post>> {
    if !Path::new(POSTS_FILE).exists() {
        return Ok(vec![]);
    }
    let file = File::open(POSTS_FILE).unwrap();
    let reader = BufReader::new(file);
    serde_json::from_reader(reader)
}

pub fn save_users(users: &[User]) -> Result<()> {
    if let Some(parent) = Path::new(USERS_FILE).parent() {
        fs::create_dir_all(parent).unwrap();
    }
    let file = File::create(USERS_FILE).unwrap();
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, users)
}

pub fn load_users() -> Result<Vec<User>> {
    if !Path::new(USERS_FILE).exists() {
        return Ok(vec![]);
    }
    let file = File::open(USERS_FILE).unwrap();
    let reader = BufReader::new(file);
    serde_json::from_reader(reader)
}
