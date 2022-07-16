//use std::collections::LinkedList;
use mio::Events;
use mio::Interest;
use std::future::Future;
use std::net::SocketAddr;
use std::task::Context;
use std::task::Poll;

pub fn spawn<T: 'static>(future: impl Future<Output = T> + 'static) {
    unimplemented!()
}

pub struct Runtime {
    //schedule_queue: LinkedList<Task>,
    poll: mio::Poll,
}

impl Runtime {
    pub fn new() -> Self {
        let poll = mio::Poll::new().unwrap();
        Runtime { poll }
    }

    pub fn block_on<T>(&mut self, future: impl Future<Output = T>) -> T {
        let waker = waker_fn::waker_fn(|| println!("woken"));
        let mut cx = Context::from_waker(&waker);

        futures::pin_mut!(future);

        if let Poll::Ready(t) = future.as_mut().poll(&mut cx) {
            println!("block_on: completed");
            return t;
        }

        let mut events = Events::with_capacity(1024);
        loop {
            self.poll.poll(&mut events, None).unwrap();
            for event in &events {
                unimplemented!();
                // match event.token() { }
            }
        }
    }
}

pub struct TcpListener {
    io: mio::net::TcpListener,
}

impl TcpListener {
    pub fn bind(addr: SocketAddr) -> std::io::Result<TcpListener> {
        let io = mio::net::TcpListener::bind(addr)?;

        Ok(TcpListener { io })
    }

    pub async fn accept(&self) -> std::io::Result<(TcpStream, SocketAddr)> {
        //futures::future::poll_fn(|cx| unimplemented!()).await?;

        /* TODO
        poll.registry()
            .register(&mut listener, Token(1024), Interest::READABLE)
            .unwrap();
        */

        if let Some(e) = self.io.take_error()? {
            return Err(e);
        }

        unimplemented!()
    }
}

pub struct TcpStream {}

impl TcpStream {
    pub async fn read(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        unimplemented!()
    }
    pub async fn write_all(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        unimplemented!()
    }
}
