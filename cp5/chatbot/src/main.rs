use tokio::net::TcpListener; // brings in Tokio's TCP server type that listens for incoming connections
use tokio::sync::broadcast; // imports a braodcast channel, which lets you send one message that multiple clients can receive at once
use std::net::SocketAddr; // struct that holds an IP address and port number

mod handler; 
mod model; 

#[tokio::main] //macro that starts the tokio runtime - basically, it sets up async multitasking 

// asycn fn main() means the function can use .await to run things concurrently without blocking 
asycn fn main() -> anyhow::Result<()> { // Result is used for return an error with context if something goes wrong, otherwise return nothing

    let listener = TcpListener::bind("127.0.0.1:6000").await?; 
    println!("Chat Server is running on 127.0.0.1:6000");

    //shared broadcast channel (retain 100 messages) 
    let (tx, _) = braodcast::channel(100); //tx is the transmiter used to send message, the underscore means we are ignoring the receiver 

    // The server loop 

    // loop { ... } runs forever — that’s the server waiting for new clients.
    // listener.accept().await waits for someone to connect to your server.
    // It returns:
    // socket → the connection to that client (used to read/write messages).
    // addr → the IP address of the client.

    loop {
        let (socket, addr) = listener.accept().await?; 

        // tx.clone() → every new client gets its own copy of the transmitter.
        // (They all still send through the same shared channel.)
        // tx.subscribe() → creates a receiver for this client, so they can read broadcasted messages from others.
        let tx = tx.clone();
        let rx = tx.subscribe(); 

    //     tokio::spawn(...) → runs this task in the background asynchronously.
    // Think of it as “launch a new mini-thread” for this user.
    // async move { ... } → move means it takes ownership of variables (socket, addr, etc.) and runs them inside this async block.
    // Inside, it calls your own function handler::handle_connection(...),
    // which likely reads messages from the client and sends them to others through the broadcast channel.
    // If it returns an error, it prints it with the user’s IP address.

        tokio::spawn(async move {
        if let Err(e) = handler::handle_connection(socket, addr, tx, rx).await {
            eprintln!("Error with {}: {:?}", addr, e);
        }
    });

    }

    
}

// What Happens When You Run It
// The server starts and listens on port 6000.
// When someone connects, the server spawns a new task for them.
// That task runs handle_connection (probably loops reading and sending messages).
// If any user sends a message, it gets broadcasted to everyone else through the tx/rx system.
// The server keeps running forever, handling multiple users concurrently.
