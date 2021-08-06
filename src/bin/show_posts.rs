extern crate diesel;
extern crate todo_cli;

use todo_cli::service::models::*;
use todo_cli::service::db::*;
use diesel::prelude::*;

fn main() {
    use todo_cli::service::schema::posts::dsl::*;

    let connection = establish_connection();
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }
}