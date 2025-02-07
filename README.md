
# Goody Chat Application 🚀  
Goody Chat App is a real-time chat application built with Rust for the client and Warp (with Tokio) for the server. 
It allows multiple users to chat simultaneously over WebSockets.  

![Screenshot 2025-02-07 132756](https://github.com/user-attachments/assets/54296fc5-697e-4ee7-94ee-be0314fd615a)
  

## ✨ Features  
- 🔄 **Real-time messaging** with WebSockets.  
- 👥 **Multi-client support** for group chat.  
- ⚡ **Asynchronous and non-blocking** message handling using Tokio.  
- 🖥️ **Simple UI** for easy interaction.  


## 📂 Project Structure  
- Client (`client/main.rs`)  
  Implements a TCP client in Rust that connects to the server. Users can send and receive messages in real time.  
- Server (`server/main.rs`)  
  Built using Warp and Tokio WebSockets. It handles client connections and broadcasts messages to all connected users.  


## 🚀 How It Works  
1. Client: Connects to the server via TCP, sends user messages, and listens for incoming messages.  
2. Server: Listens for client connections, manages connected clients, and broadcasts messages to all clients.  


## ⚙️ Setup and Run Instructions  

## Prerequisites  
- [Rust](https://www.rust-lang.org/tools/install) and Cargo installed.  
- Tokio and Warp dependencies (already included in `Cargo.toml`).  

### Running the Project  

1. **Clone the repository**:  
   ```sh
   git clone https://github.com/Anish2324/Goody-Multiple-Chat-Application.git
   
2. **Start the Server**:  
   ```sh
   cd server  
   cargo run  
     
3.**Open your web browser**:
  Go to http://127.0.0.1:8080/chat.
  Start sending messages!
  

4. **Chat!** 🎉  
   Type your message in the client console and see it broadcasted to all connected users.  


## 🛠️ Tech Stack  
- Languages:Rust,HTML  
- Frameworks: Warp (Server), Tokio (Asynchronous Runtime)  
- WebSocket Library: tokio-tungstenite  

## 🚀 Future Improvements  
- Add user authentication.  
- Store chat history in a database.  
- Improve the UI for a more modern chat experience.  


## 🤝 Contributing  
Contributions, issues, and feature requests are welcome!  

## 🧑‍💻 Author  
Goody Chat Application by Anish D Tabib(https://github.com/Anish2324)  
