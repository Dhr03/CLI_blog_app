mod post;
mod storage;
mod cli;

use post::{Category, Post};
use storage::{load_posts, save_posts};
use cli::{Cli, Commands};

use clap::Parser;

/// Convert a string (like "tech") to the Category enum
fn parse_category(cat: &str) -> Category {
    match cat.to_lowercase().as_str() {
        "tech" => Category::Tech,
        "lifestyle" => Category::Lifestyle,
        "travel" => Category::Travel,
        _ => Category::Uncategorized,
    }
}

fn main() {
    let args = Cli::parse(); // Parse command-line args
    let mut posts = load_posts().expect("Failed to load posts");

    match args.command {
        Commands::Add {
            title,
            content,
            category,
            author,
        } => {
            let post = Post {
                id: posts.len() as u32 + 1,
                title,
                content,
                category: parse_category(&category),
                author,
            };
            posts.push(post);
            save_posts(&posts).expect("Failed to save posts");
            println!("Post added!");
        }

        Commands::List => {
            if posts.is_empty() {
                println!("No posts yet.");
            } else {
                for post in posts {
                    println!(
                        "{} | {} | {:?} | by {}",
                        post.id, post.title, post.category, post.author
                    );
                }
            }
        }

        Commands::Clear => {
            posts.clear();
            save_posts(&posts).expect("Failed to save posts");
            println!("All posts cleared.");
        }
    }
}
