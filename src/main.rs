mod post;
mod storage;
mod cli;
mod user;
mod comment;

use post::{Category, Post};
use user::User;
use comment::Comment;
use storage::{load_posts, save_posts, load_users, save_users, load_comments, save_comments};
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

        Commands::Comment {
            post_id, 
            content, 
            author, 
        } => {
            let mut comments = load_comments().expect("Failed to load comments");

            if !posts.iter().any(|p| p.id==post_id) {
                println!("No post found with id {post_id}");
                return ;
            }
            let user_id = get_or_create_user(author, &mut users);

            let comment = Comment {
                id: comments.len() as u32 + 1,
                post_id, 
                user_id, 
                content, 
            };

            comments.push(comment);
            save_comments(&comments).expect("Failed to save comments");
            save_users(&users).expect("Failed to save users");

            println!("Comment added.");
        }

        Commands::List {with_comments} => {
            let comments = load_comments().expect("Failed to load comments");
            if posts.is_empty() {
                println!("No posts yet.");
            } 
            else {
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
                    println!("\t{}", post.content);

                    if with_comments {
                        let post_comments: Vec<&Comment> = comments
                            .iter()
                            .filter(|c| c.post_id == post.id)
                            .collect();
                        
                        if post_comments.is_empty() {
                            println!("    💬 No comments yet.");
                        }
                        else {
                            for comment in post_comments {
                                let commenter = users
                                    .iter()
                                    .find(|u| u.id == comment.user_id)
                                    .map(|u| u.name.clone())
                                    .unwrap_or("Anonymus".to_string());
                                
                                println!("      - {}: {}", commenter, comment.content);
                            }
                        }
                    }
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
