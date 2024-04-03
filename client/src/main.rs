use std::{io, thread, str};
use std::io::{ErrorKind,Read,Write};
use std::sync::mpsc::{self, TryRecvError};
use std::net::TcpStream;

const MSG_SIZE:usize = 64;
const LOCAL: &str = "127.0.0.1:7878";

fn sleep() {thread::sleep(::std::time::Duration::from_millis(500));} 

fn main() {
    let mut client = TcpStream::connect(LOCAL).expect("failed to bind");
    client.set_nonblocking(true).expect("failed to set non-blocking");

    let (tx,rx) = mpsc::channel::<String>();
    println!("Please enter a username");
    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("reading from stdin failed");

    thread::spawn(move || loop{
        let mut buff = vec![0;MSG_SIZE];
        match client.read_exact(&mut buff){
            Ok(_) => {
                let msg = buff.into_iter().take_while(|&x| x!= 0).collect::<Vec<_>>();
                let msg = str::from_utf8(&msg).unwrap();
                println!("message received: {:?}",msg);
            }
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                println!("connection with server was severed");
                break;
            }
        }
        match rx.try_recv(){
            Ok(msg)=> {
                let mut buff = msg.clone().into_bytes();
                buff.resize(MSG_SIZE, 0);
                client.write_all(&buff).expect("writing to socket failed");
                println!("message sent {:?} from {username}",msg);
            },
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break
        }
        sleep();
    });
    println!("Write a message: \n");
    loop{
        let mut buff = String::new();
        io::stdin().read_line(&mut buff).expect("reading from stdin failed");
        let msg = buff.trim().to_string();
        if msg == ":quit" || tx.send(msg).is_err() {break};
    }
    println!("chat terminated");
}
