mod post;
mod storage;
mod cli;
mod user;

use post::{Category, Post};
use user::User;
use storage::{load_posts, save_posts, load_users, save_users};
use cli::{Cli, Commands};
use clap::Parser;

/// Utility: Converts category string to enum
fn parse_category(cat: &str) -> Category {
    match cat.to_lowercase().as_str() {
        "tech" => Category::Tech,
        "lifestyle" => Category::Lifestyle,
        "travel" => Category::Travel,
        _ => Category::Uncategorized,
    }
}

/// Utility: Find user by name, or create one
fn get_or_create_user(name: String, users: &mut Vec<User>) -> u32 {
    if let Some(user) = users.iter().find(|u| u.name == name) {
        return user.id;
    }

    let new_id = users.len() as u32 + 1;
    let new_user = User {
        id: new_id,
        name,
        email: format!("user{}@example.com", new_id), // placeholder
    };

    users.push(new_user);
    new_id
}

fn main() {
    let args = Cli::parse();
    let mut posts = load_posts().expect("Failed to load posts");
    let mut users = load_users().expect("Failed to load users");

    match args.command {
        Commands::Add {
            title,
            content,
            author,
            category,
        } => {
            let user_id = get_or_create_user(author, &mut users);

            let post = Post {
                id: posts.len() as u32 + 1,
                title,
                content,
                category: parse_category(&category),
                user_id,
            };

            posts.push(post);
            save_posts(&posts).expect("Failed to save posts");
            save_users(&users).expect("Failed to save users");
            println!("Post added!");
        }

        Commands::List => {
            if posts.is_empty() {
                println!("No posts yet.");
            } else {
                for post in &posts {
                    let author_name = users
                        .iter()
                        .find(|u| u.id == post.user_id)
                        .map(|u| u.name.clone())
                        .unwrap_or("Unknown".to_string());

                    println!(
                        "{} | {} | {:?} | by {}",
                        post.id, post.title, post.category, author_name
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
