# Command Line Blog app
There is no auth currenty. All users can see and edit all users post (am planning to change it in future!)

- Package manager: cargo

# How to use: 
1. Clone the repo.
2. Navigate to the folder
3. excecute this command: `cargo build`. This installs all the required crates
4. Commands accepted:
    1. list all posts: `cargo run -- list`
    2. list all posts with comments: `cargo run -- list --with-comments
    3. add a new post: `cargo run -- add <post-title> <description> <category> <author>`
    4. edit a post `cargo run -- edit <post_id> [--title "New Title"] [--content "New Content"] [--category "NewCategory"]`
    5. add a comment under a post: `cargo run -- comment <post-id> <comment> <author>`
    6. delete a post: `cargo run -- delete <post_id>`
