use warp::Filter;
use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite::Message;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use futures_util::{StreamExt, SinkExt};

type Clients = Arc<Mutex<HashMap<usize, mpsc::UnboundedSender<Message>>>>;
static USER_ID_COUNTER: AtomicUsize = AtomicUsize::new(1); // Unique user ID generator

#[tokio::main]
async fn main() {
    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));

    let chat = warp::path("chat")
        .and(warp::ws())
        .map(move |ws: warp::ws::Ws| {
            let clients = clients.clone();
            ws.on_upgrade(move |socket| handle_connection(socket, clients))
        });

    warp::serve(chat).run(([127, 0, 0, 1], 8080)).await;
}

async fn handle_connection(ws: warp::ws::WebSocket, clients: Clients) {
    let user_id = USER_ID_COUNTER.fetch_add(1, Ordering::Relaxed); // Generate unique ID
    let (mut sender, mut receiver) = ws.split();

    let (tx, mut rx) = mpsc::unbounded_channel();

    // Store the client in the HashMap
    clients.lock().unwrap().insert(user_id, tx.clone());

    println!("User {} connected", user_id);

    // Listen for messages from this client
    let clients_clone = clients.clone();
    tokio::spawn(async move {
        while let Some(result) = receiver.next().await {
            if let Ok(msg) = result {
                if msg.is_text() {
                    let text = format!("User {}: {}", user_id, msg.to_str().unwrap());
                    println!("{}", text);

                    // Broadcast the message to all clients
                    let clients = clients_clone.lock().unwrap();
                    for (_, client) in clients.iter() {
                        let _ = client.send(Message::text(text.clone()));
                    }
                }
            }
        }

        // Remove client on disconnect
        clients_clone.lock().unwrap().remove(&user_id);
        println!("User {} disconnected", user_id);
    });

    // Send messages from the queue to the client
    while let Some(msg) = rx.recv().await {
        if sender.send(warp::ws::Message::text(msg.to_text().unwrap())).await.is_err() {
            break;
        }
    }
}
