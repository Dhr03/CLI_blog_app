mod post;
mod storage;

use post::{Category, Post};
use storage::{save_posts, load_posts};

fn main() {
    // Load all the existing posts
    let mut posts = load_posts().expect("Failed to load posts");

    let new_post = Post {
        id: posts.len() as u32 + 1, 
        title: "New Post Test".to_string(), 
        content: "This is a CLI Blogging platform.\nIt has file storage as well".to_string(), 
        category: Category::Uncategorized, 
        author: "Dhruv".to_string(), 
    };

    // Add new post to the list
    posts.push(new_post);

    // Save it to the disk
    save_posts(&posts).expect("Failed to save posts");

    // Print to the screen
    println!("All posts: ");
    for post in posts {
        println!("{}: {}", post.id, post.title);
    }
}
