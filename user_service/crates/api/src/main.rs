use actix_web:: {web, App, HttpServer}; 
use infrastructure::InMemoryUserRepo;
use std::sync::Arc; 
use handler::{register_user, get_user};

mod handler;
// Why Arc<>?
// Your HTTP server runs multiple worker threads (Actix does this for performance).
// All handlers need to share the same repository instance (your in-memory DB) so requests on different threads see the same data.
// Arc<T> = Atomic Reference-Counted smart pointer:
// Many clones of the pointer → one shared T on the heap.
// Thread-safe (atomic ref counts) so it works across threads.
// When the last Arc is dropped, the inner value is freed.

pub struct AppState {
    pub repo: Arc<InMemoryUserRepo>
}




// Server (HttpServer + App):
// Listens on 127.0.0.1:8080.
// Knows the routes (which URL + method goes to which handler).
// Holds shared stuff (your AppState with the repo) so handlers can use it.
// Manages worker threads and the async runtime.

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let repo = Arc::new(InMemoryUserRepo::new());

    HttpServer::new(move || {
        // this closure runs per worker; we CLONE a pointer (Arc) to share the repo safely
        App::new()
            // web::Data is actix’s way to share state with handlers (thread-safe)
            .app_data(web::Data::new(AppState {
                repo: Arc::clone(&repo),
            }))
            // register routes (path + method → handler fn)
            .route("/register", web::post().to(register_user))
            .route("/user/{username}", web::get().to(get_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}