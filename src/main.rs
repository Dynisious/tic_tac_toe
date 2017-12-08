//! Author --- daniel.bechaz@gmail.com  
//! Last Modified --- 2017/12/5

mod squares;
mod board;
mod communication;

use squares::*;
use board::*;
use std::net::TcpListener;
use std::thread;
use std::io::{Read, Write};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:1234").unwrap();
    
    println!("Listening on {:?}...", listener.local_addr().unwrap());
    listener.incoming() //This gets an infinite iterator, it will block untill a new connection is available.
    .for_each(|stream| {
        match stream {
            //The connection was successful.
            Ok(mut stream) => {
                eprintln!("New connection: {:?}", stream.peer_addr().unwrap());
                //Spawn a new thread to replay to this connection.
                thread::spawn(move || {
                    //Attempt to write message to the connection...
                    if let Err(e) = stream.write_all("Hello TCP/IP!!!".as_bytes()) {
                        //If there was an error writing, print error message.
                        eprintln!("Failed write: {}", e);
                    }
                    eprintln!("Connection closed: {:?}", stream.peer_addr().unwrap());
                });
            },
            //The connection failed.
            Err(e) => eprintln!("Failed connection: {}", e)
        }
    });
}
