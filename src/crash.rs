use std::{
    io::{Read, Write},
    net::TcpStream,
};

use crate::URL;

pub fn crash() {
    println!("Launching crash ");
    let mut stream = TcpStream::connect(URL).unwrap();

    let mut buf = [];
    while true {
        stream.write("SIZE".as_bytes()).expect("Could not write");
        stream.read(&mut buf).expect("crash");
        stream.read(&mut buf).expect("crash");

        stream
            .write("PX 5 5 ffffff".as_bytes())
            .expect("Could not write");
    }
}
