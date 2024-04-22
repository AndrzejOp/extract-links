use std::io::{self, ErrorKind, Read,Write};
use std::net::TcpStream;
use std::sync::mpsc::{self,TryRecvError};
use std::thread;
use std::time::Duration;


const MSG_SIZE: usize = 64;
const LOCAL: &str = "127.0.0.1:6000";

fn main() {
    let mut client = TcpStream::connect(LOCAL).expect("Failed to connect stream");
    client.set_nonblocking(true).expect("Non-blocking failed");

    let (tx,rx) = mpsc::channel::<String>();

    thread::spawn(move || loop {
        let mut buff = vec![0; MSG_SIZE];
        match client.read_exact(&mut buff) {
            Ok(_) => {
                let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                println!("received message: {:?}", msg);
            },
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                println!("Lost connection");
                break;
            }
        }
        match rx.try_recv() {
            Ok(msg) => {
                let mut buff = msg.clone().into_bytes();
                buff.resize(MSG_SIZE, 0);
                client.write_all(&buff).expect("Writing to socket failed");
                println!("sent message: {:?}", msg);
            },
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break
        }
        thread::sleep(Duration::from_millis(200));
    });
    println!("Write first message: ");
    loop {
        let mut buff = String::new();
        io::stdin().read_line(&mut buff).expect("reading failed");
        let msg = buff.trim().to_string();
        if msg == ":quit" || tx.send(msg).is_err() {break}
    }
    println!("CYA");
}
