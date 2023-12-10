use std::{
    fmt::format,
    io::{Read, Write},
    net::TcpStream,
    time::Duration,
};

use image::{io::Reader as ImageReader, GenericImageView};
use std::io::Cursor;

use crate::URL;

const PADDING_X: u32 = 0;
const PADDING_Y: u32 = 0;

pub fn fond(path: Option<String>) {
    let path = path.unwrap_or("ben.png".to_string());

    let mut stream = TcpStream::connect(URL).unwrap();
    println!("Connected");

    if true {
        while true {
            let img = ImageReader::open(&path).unwrap().decode();
            match img {
                Ok(img) => {
                    for (x, y, pixel) in img.pixels() {
                        let p = pixel.0;
                        let string = format!(
                            "PX {} {} {:02x}{:02x}{:02x}\n",
                            x + PADDING_X,
                            y + PADDING_Y,
                            p[0],
                            p[1],
                            p[2]
                        );
                        let buf = string.as_bytes();

                        let result = stream.write(&buf);
                        if result.is_err() {
                            println!("Result {:#?}", result);
                            panic!("result {:#?}", result.err());
                        }
                    }
                }
                Err(_) => println!("Error"),
            };
        }
    }
    while true {
        for i in 0..600 {
            for j in 0..500 {
                let string = format!("PX {} {} {}\r\n", i, j, "ffeeff");

                //print!("Command: {}", string);

                let buf = string.as_bytes();
                let result = stream.write(&buf);

                if result.is_err() {
                    println!("Result {:#?}", result);
                }
            }
        }

        /*
        stream.write("HELP\n".as_bytes());
        let read_stream = stream.read(&mut buf);
        if read_stream.is_ok() {
            let string = String::from_utf8_lossy(&buf);
            //    println!("Result {:#?}", string)
          }
          */
    }
}
