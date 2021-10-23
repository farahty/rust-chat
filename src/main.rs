use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();
    let (tx, _) = broadcast::channel(10);

    println!("chat server is ready");
    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();
        println!("clint with addr {} is connected", addr);
        let tx = tx.clone();
        let mut rx = tx.subscribe();
        tokio::spawn(async move {
            let (read, mut write) = socket.split();
            let mut reader = BufReader::new(read);
            let mut line = String::new();
            loop {
                tokio::select! {
                    results =  reader.read_line(&mut line) => {
                        if results.unwrap() == 0 {
                            println!("good bye :) client {}", &addr);
                            break;
                        }
                        tx.send((line.clone(), addr)).unwrap();
                        print!("client:{} message: {}", addr, line);
                        line.clear();
                    }
                    results = rx.recv() => {
                        let (message, other_addr) = results.unwrap();
                        if addr != other_addr {
                            write.write_all(message.as_bytes()).await.unwrap();
                        }
                    }
                }
            }
        });
    }
}
