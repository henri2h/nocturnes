use std::{
    fmt::format,
    io::{Read, Write},
    net::TcpStream,
    time::Duration,
};

use image::{io::Reader as ImageReader, GenericImageView};
use std::io::Cursor;

use crate::URL;

const SIZE_X: i32 = 1024;
const SIZE_Y: i32 = 575;

const X_SPEED: i32 = 60;
const Y_SPEED: i32 = 40;

pub fn diag() {
    let mut mode_rev_x = false;
    let mut mode_rev_y = false;
    let mut mode = false;

    let img_federez = ImageReader::open("federez.jpeg").unwrap().decode().unwrap();
    let img_minitel = ImageReader::open("minitel.png").unwrap().decode().unwrap();

    let mut stream = TcpStream::connect(URL).unwrap();
    println!("Connected");

    let mut i: i32 = 0;
    let mut j: i32 = 0;

    while true {
        let img = match mode {
            true => &img_federez,
            false => &img_minitel,
        };
        mode = !mode;

        let (w, h) = img.dimensions();

        let max_size_x = SIZE_X - (w as i32) - 1;
        let max_size_y = SIZE_Y - (h as i32) - 1;

        if mode_rev_x {
            i -= X_SPEED;
            if i < 0 {
                mode_rev_x = false;
                i = 0;
            }
        } else {
            i += X_SPEED;
            if i > max_size_x {
                mode_rev_x = true;
                i = max_size_x;
            }
        }

        if mode_rev_y {
            j -= Y_SPEED;
            if j < 0 {
                mode_rev_y = false;
                j = 0;
            }
        } else {
            j += Y_SPEED;
            if j > max_size_y {
                mode_rev_y = true;
                j = max_size_y;
            }
        }

        if j < 0 {
            j = 0;
        }
        if i < 0 {
            i = 0;
        }

        for (x, y, pixel) in img.pixels() {
            let p = pixel.0;
            let string = format!(
                "PX {} {} {:02x}{:02x}{:02x}\n",
                x + (i as u32),
                y + (j as u32),
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
}
