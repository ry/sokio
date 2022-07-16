mod sokio;
//use sokio::AsyncReadExt;
//use sokio::AsyncWriteExt;

async fn process_socket(mut socket: sokio::TcpStream) {
    let mut req = [0; 4096];
    let res = b"HTTP/1.1 200 OK\r\nContent-length: 12\r\n\r\nHello world\n";

    loop {
        let n = socket.read(&mut req).await.unwrap();
        if n == 0 {
            return;
        }
        socket.write_all(res).await.unwrap();
    }
}

fn main() {
    let mut rt = sokio::Runtime::new();
    rt.block_on(async {
        let addr = "127.0.0.1:9000".parse().unwrap();
        let listener = sokio::TcpListener::bind(addr).unwrap();

        loop {
            let (socket, _) = listener.accept().await.unwrap();
            sokio::spawn(process_socket(socket));
        }
    });
}
