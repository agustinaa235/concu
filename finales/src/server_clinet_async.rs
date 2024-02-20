
/*
    Hacer un servidor TCP asincronico en donde cada 
    cliente que se conecta, el servidor le responde el
    mensaje que envia hasta que el cliente envia 'S
*/

use std::net::SocketAddr;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, split};
use tokio::net::{TcpListener, TcpStream};

async fn main(){
    let listener = TcpListener::bind("127.0.0.1:12345").await.unwrap();

    println!("[SERVER] esperando conexiones");

    loop {
        if let Ok((mut stream, addr)) = listener.accept().await{
            println!("[{:?}] Cliente conectado", addr);
            tokio::spawn(async move {
                receiver(stream, addr).await;
            });

        } else {
            println!("Error de conexion de cliente ");
        }
    }
}

async fn receiver(mut stream: TcpStream, addr: SocketAddr){
    let (r, mut w) = split(stream);
    let mut reader = BufReader::new(r);
    loop {
        let mut buffer = String::new();
        if let Ok(read) = reader.read_line(&mut buffer).await{
            if read > 0 {
                println!("[{:?}] Message: {}", addr, buffer);
                if buffer ==  'S'.to_string() {
                    println!("[{:?}] Goodbye!", addr);
                    break;
                } else {
                    w.write_all(format!("{}", buffer).as_bytes()).await
                    .expect("");
                }  
            } else {
                println!("[{:?}] Error leyendo socket!", addr);
                break;
            }
        }
    }
}


