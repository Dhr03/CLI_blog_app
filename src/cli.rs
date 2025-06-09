use clap::{Parser, Subcommand};

/// CLI Blog Application
#[derive(Parser)]
#[command(name = "CLI Blog")]
#[command(about = "A simple CLI-based blogging platform", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Subcommands the user can invoke
#[derive(Subcommand)]
pub enum Commands {
    /// Add a new blog post
    Add {
        title: String,
        content: String,
        author: String,
        #[arg(default_value = "Uncategorized")]
        category: String,
    },

    /// Add a commet to a post
    Comment {
        post_id: u32, 
        content: String, 
        author: String, 
    }, 

    /// List all blog posts
    List {
        #[arg(short, long, default_value_t=false)]
        with_comments: bool, 
    },

    /// Clear all posts
    Clear,
}
