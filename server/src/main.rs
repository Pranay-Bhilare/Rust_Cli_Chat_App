use::tokio::net::{TcpListener,TcpStream};
use::tokio::io::{self,AsyncReadExt,AsyncWriteExt};
use::tokio::sync::broadcast;
// use::std::sync::Arc;

const ADDRESS: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;

// #[tokio::main]
async fn main() {
    let listener = TcpListener::bind(ADDRESS).await.expect("Failed to bind");
    let (tx,_rx) = broadcast::channel(16);
    let mut next_id: usize = 0;
    loop{
        tokio::select! {
        Ok((mut socket, addr)) = listener.accept() => {
           let rx = tx.subscribe();

           tokio::spawn(handle_client(socket, tx.clone(),rx,next_id,addr));
           next_id += 1;

        }}
    }
}

async fn handle_client(
    mut socket: TcpStream,
    tx: broadcast::Sender<(usize, &[u8])>,
    mut rx: broadcast::Receiver<(u8,&[u8])>,
    id: usize,
    addr: std::net::SocketAddr
    ){
    let mut buffer = vec![0;MSG_SIZE];
    std_socket = socket.into_std();
    if let Err(e) = tx.send((id, String::from("has joined the chat"), addr)) {
        warn!("Failed to announce new client {}", e);
    }

    loop{
        tokio::select! {
            result = std_socket.read_exact(&mut buffer) => {
                match result { 
                    Ok(_) => {
                        let msg = String::from_utf8_lossy(&buffer).trim_end_matches(|c| c == '\0' || c == ' ').to_string();
                        if msg == :quit {
                            break;
                        }

                        // Info daalo ki konsa client agaya hai
                        if let Errr(e) = tx.send((id,msg,addr)){
                            warn!("Error broadcasting message from cleint {} {]",id, e);
                        }
                    },

                    Err(e) => {
                        error!("Failed to read from client {} ({}): {}", id, addr, e);
                        break;
                    }
                }
            },

            result = rx.recv() => {
                //  prevents a client from receiving back its own messages.
                if Ok(sender_id,msg,sender_addr) = result { 
                    if sender_id != id { 
                        let display_msg = format!("[Client {}]: {}", sender_id, msg);
                        let bytes_buffer = String::into_bytes(display_msg);
                        bytes_buffer.resize(MSG_SIZE,0);

                       if Err(e) = socket.write_all(&bytes_buffer).await { 
                            error!("Failed to send message to client {} {} : {}", id, addr, e);
                            break;
                        }
                    }
                }

                }
            }
        }

}