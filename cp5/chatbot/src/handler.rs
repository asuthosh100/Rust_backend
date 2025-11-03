use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
    sync::broadcast::{Sender, Receiver},
};
use crate::model::ChatMessage;
use std::net::SocketAddr;

// Import	Purpose
// tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader}	Async I/O helpers — to read and write from the socket without blocking
// tokio::net::TcpStream	Represents a single connection between your server and one client
// tokio::sync::broadcast::{Sender, Receiver}	The chat channel: Sender sends messages, Receivers get them
// crate::model::ChatMessage	Imports your ChatMessage struct from model.rs
// std::net::SocketAddr	Type representing IP + port of a user (like “127.0.0.1:51234”)

pub async fn handle_connection(
    stream: TcpStream,
    addr: SocketAddr,
    tx: Sender<ChatMessage>,
    mut rx: Receiver<ChatMessage>,
) -> anyhow::Result<()> {

// pub → this function is public, so main.rs can call it.
// async fn → runs asynchronously (non-blocking).
// stream → the connection for this particular client.
// addr → the client’s address (so we know who’s who).
// tx → the shared broadcast sender — used to send messages out.
// rx → this client’s receiver — used to receive messages from others.
// -> anyhow::Result<()> → returns Ok(()) if everything’s fine, or an error otherwise.

let (reader, mut writer) = stream.into_split();
let mut reader = BufReader::new(reader).lines();

// stream.into_split() → splits one TCP connection into:
// a reader part (to read incoming text)
// a writer part (to send outgoing text)
// BufReader::new(reader) → wraps the reader so we can read it line by line.
// .lines() → turns it into an iterator of text lines (each message typed by the user).
// So now:
// reader = reads messages from this client
// writer = sends messages to this client


writer.write_all(b"Welcome to Tokio Chat!\n").await?;

// Sends a small greeting to the new client.
// b"..." means it’s a byte string (since sockets send bytes).
// .await? → wait until it’s sent; return an error if it fails.

let mut write_task = tokio::spawn(async move {
    while let Ok(msg) = rx.recv().await {
        if msg.sender != addr {
            let line = format!("{} says: {}\n", msg.sender, msg.content);
            if let Err(_) = writer.write_all(line.as_bytes()).await {
                break;
            }
        }
    }
});

// tokio::spawn(async move { ... }) → runs this loop in the background.
// rx.recv().await → waits for the next broadcasted message.
// if msg.sender != addr → don’t send your own messages back to yourself.
// format!(...) → create a pretty message like 127.0.0.1:54321 says: Hello!
// writer.write_all(...) → send that message to this user’s socket.
// If writing fails (like the user disconnected), break exits the loop.
// 🧩 So this task constantly listens for other people’s messages and prints them here.

while let Some(Ok(line)) = reader.next_line().await {
    let msg = ChatMessage {
        sender: addr,
        content: line,
    };
    let _ = tx.send(msg);
}
// Now we handle the messages this user sends.
// reader.next_line().await → waits for the user to type a line of text and press Enter.
// ChatMessage { sender: addr, content: line } → create a new ChatMessage struct.
// tx.send(msg) → send it to the broadcast channel — this automatically delivers it to everyone’s receiver.

// 🧩 Step 7 — Clean Up When Disconnected
write_task.abort();
println!("{} disconnected", addr);
Ok(())
// write_task.abort() → stops the background task that was writing messages to this user.
// (No need to keep it running if they left.)
// Then we log that the client disconnected.
}