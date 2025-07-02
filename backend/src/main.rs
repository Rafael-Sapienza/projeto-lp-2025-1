use actix_files::Files;
use actix_web::{App, HttpServer};

mod handlers;
mod ir;
mod models;
mod routes;
mod parser;
mod environment;
mod type_checker;
mod interpreter;

// Allows the main function to be an async function
#[actix_web::main]
// The function return a Result. Which Result depends on the the setting / execution of the server
async fn main() -> std::io::Result<()> {
    const ADDRESS: &str = "127.0.0.1";
    const PORT: u16 = 8080;

    // HttpServer returns a builder that configures the server
    // It takes a closure function
    let server = HttpServer::new(|| {
        // Creates a new Actix Web application factory
        // Each thread will call that closure once to build its own App
        // It allows each thread to have its own separate state if needed
        // Its the value passed to HttpServer::new
        App::new()
            // Accepts a function/closure that configures multiple routes/services
            .configure(routes::register_routes)
            // Mounts the / path to the folder ../frontend on disk
            // If a user visits /, Actix will return ../frontend/index.html
            .service(Files::new("/", "../frontend").index_file("index.html"))
    })
    // Returns a Result<ServerBuilder, std::io::Error>
    // "?": In case of an error (like if the port is not available) returns an error
    // Spawns runtime + workers
    // Each worker: App::new()
    .bind((ADDRESS, PORT))?
    // Returns a future that represents the running server, but doesn’t actually start it yet.
    .run();

    println!("Server running on port {}:{}", ADDRESS, PORT);
    // Starts the event loop, accepts connections, and runs the server.
    server.await
}
