use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc;
use std::io::{self, BufRead};
use std::net::TcpStream;
use std::thread;
use tracing::{info, error};
use tracing_subscriber;

const SERVER_ADDR: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;

#[tokio::main]
async fn main(){
    tracing_subscriber::fmt::init();

    let socket = match TcpStream::connect(SERVER_ADDR).await { 
        Ok(socket) => { 
            info!("Connected to server {}", SERVER_ADDR);
            socket
        },
        Err(e) => { 
            error!("Failed to connec to server {} : {}", SERVER_ADDR, e);
            return;
        }
    };

    let (tx, mut rx) = mpsc::channel(16);
    let(mut reader, mut writer) = socket.into_split();

    let read_task = tokio::spawn(async move {
        let mut buffer = [0; MSG_SIZE];
        loop { 
            match reader.read_exact(&mut buffer).await { 
                Ok(_) => { 
                    let msg = buffer.iter().take_while(|&&x| x!=0).copied().collect::<Vec<u8>>();
                
                    if let Ok(text) = String::from_utf8(msg) {
                        info!("{}", text);
                    } else {
                        info!("Received message (non-UTF8)");
                    }
                },
                Err(e) => {
                    error!("Connection error: {}", e);
                    break;
                }
            }
        }
    });

    let tx_clone = tx.clone();
    thread::spawn(move || {
        println!("Chat started. Type your messages (or ':quit' to exit):");
        let stdin = io::stdin();
        let mut lines = stdin.lock().lines();
        
        while let Some(Ok(line)) = lines.next() {
            let message = line.trim().to_string();

            if tx_clone.blocking_send(message.clone()).is_err() {
                break;
            }

            if message == ":quit" {
                break;
            }
        }
    });
    let write_task = tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            let mut send_buffer = message.clone().into_bytes();
            send_buffer.resize(MSG_SIZE, 0);
            
            if let Err(e) = writer.write_all(&send_buffer).await {
                eprintln!("Failed to send message: {}", e);
                break;
            }
            
            if message == ":quit" {
                break;
            }
        }
    });
    tokio::select! {
        _ = read_task => println!("Server connection closed"),
        _ = write_task => println!("Chat session ended by user"),
    }
    
    println!("Disconnected from server. Goodbye!");
}