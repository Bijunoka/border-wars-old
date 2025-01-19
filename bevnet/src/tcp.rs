use std::{
    io::{self, Read, Write},
    net::{Shutdown, TcpStream, ToSocketAddrs},
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
};

#[cfg(feature = "server")]
use std::net::TcpListener;

/// A non-blocking TCP connection.
pub struct Connection {
    /// Track if the connection has been closed.
    closed: Arc<AtomicBool>,

    /// The underlying TCP stream.
    stream: TcpStream,

    /// Used to receive packets from the receiving thread.
    receiver: Mutex<Receiver<Vec<u8>>>,

    /// Used to send packets to the sending thread.
    sender: Mutex<Sender<Vec<u8>>>,
}

impl Connection {
    /// Creates a new connection.
    fn new(stream: TcpStream) -> io::Result<Self> {
        stream.set_nonblocking(false)?;
        let closed = Arc::new(AtomicBool::new(false));

        // Spawn the receiving thread
        let thread_stream = stream.try_clone()?;
        let (thread_sender, receiver) = channel();
        let thread_closed = Arc::clone(&closed);
        thread::spawn(move || Self::receiving_loop(thread_stream, thread_sender, thread_closed));

        // Spawn the sending thread
        let thread_stream = stream.try_clone()?;
        let (sender, thread_receiver) = channel();
        let thread_closed = Arc::clone(&closed);
        thread::spawn(move || Self::sending_loop(thread_stream, thread_receiver, thread_closed));

        // Return the connection
        Ok(Self {
            closed,
            stream,
            receiver: Mutex::new(receiver),
            sender: Mutex::new(sender),
        })
    }

    #[cfg(feature = "client")]
    /// Creates a new connection to the given address.
    pub fn connect<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        Self::new(TcpStream::connect(addr)?)
    }

    /// The receiving loop for this connection.
    fn receiving_loop(mut stream: TcpStream, sender: Sender<Vec<u8>>, closed: Arc<AtomicBool>) {
        let mut len_buffer = [0; 4];
        loop {
            // Read the length of the next packet
            if stream.read_exact(&mut len_buffer).is_err() {
                break;
            }
            let len = u32::from_be_bytes(len_buffer);

            // Read the packet
            let mut packet = vec![0; len as usize];
            if stream.read_exact(&mut packet).is_err() {
                break;
            }

            // Send the packet
            if sender.send(packet).is_err() {
                break;
            }
        }
        closed.store(true, Ordering::Relaxed);
    }

    /// The sending loop for this connection.
    fn sending_loop(mut stream: TcpStream, receiver: Receiver<Vec<u8>>, closed: Arc<AtomicBool>) {
        while let Ok(packet) = receiver.recv() {
            // Send the length of the packet
            let len_buffer = u32::to_be_bytes(packet.len() as u32);
            if stream.write_all(&len_buffer).is_err() {
                break;
            }

            // Send the packet
            if stream.write_all(&packet).is_err() {
                break;
            }
        }
        closed.store(true, Ordering::Relaxed);
    }

    /// Returns `true` if the connection has been closed.
    pub fn closed(&self) -> bool {
        self.closed.load(Ordering::Relaxed)
    }

    /// Returns the next received packet.
    pub fn recv(&self) -> Option<Vec<u8>> {
        self.receiver
            .lock()
            .ok()
            .and_then(|receiver| receiver.try_recv().ok())
    }

    /// Sends a packet through this connection.
    pub fn send(&self, packet: Vec<u8>) {
        self.sender.lock().map(|sender| sender.send(packet)).ok();
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        self.stream.shutdown(Shutdown::Both).ok();
    }
}

#[cfg(feature = "server")]
/// A [Connection] listener.
pub struct Listener {
    /// The [TcpListener] of the listener.
    listener: TcpListener,
}

#[cfg(feature = "server")]
impl Listener {
    /// Creates a new TCP listener on the given address.
    pub fn bind<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        let listener = TcpListener::bind(addr)?;
        listener.set_nonblocking(true)?;
        Ok(Self { listener })
    }

    /// Accepts a new [Connection].
    pub fn accept(&self) -> Option<Connection> {
        self.listener
            .accept()
            .and_then(|(stream, _)| Connection::new(stream))
            .ok()
    }
}
