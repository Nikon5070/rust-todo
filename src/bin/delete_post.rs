extern crate diesel;
extern crate todo_cli;

use std::env::args;
use diesel::prelude::*;
use todo_cli::service::{db::*};

fn main() {
    use todo_cli::service::schema::posts::dsl::*;

    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{}%", target);

    let connection = establish_connection();
    let num_deleted = diesel::delete(posts.filter(title.like(pattern)))
        .execute(&connection)
        .expect("Error deleting posts");

    println!("Deleted {} posts", num_deleted);
}

