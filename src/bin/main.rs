#[macro_use]
extern crate explore;
extern crate byteorder;

use std::io;
use std::fs::File;

fn main() {

    let mut file = File::create("target/foo.txt")
        .expect("Could not open file");

    loop {
        println!("Enter a message");

        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        if input.trim().len() == 0 {
            break;
        }

        write_to_file(&mut file, &mut input)
    }

}

fn write_to_file(file: &mut File, msg: &str) {
    use std::io::prelude::*;
    use byteorder::{ByteOrder, BigEndian};

    let bytes = msg.as_bytes();
    let length = bytes.len() as u32;
    let mut ba = [0u8; 4];
    BigEndian::write_u32(&mut ba, length);

    println!("write length: {}", length);

    file.write_all(&ba).unwrap();
    file.write_all(msg.as_bytes()).unwrap();
}

fn read_from_file() -> String {
    use std::fs::File;
    use std::io::Read;
    use std::str::from_utf8;
    use byteorder::{ByteOrder, BigEndian};

    let mut f = File::open("target/foo.txt").unwrap();

    let mut ba = [0u8; 4];
    f.read_exact(&mut ba).unwrap();

    let length = BigEndian::read_u32(&mut ba) as usize;
    println!("read length: {}", length);

    let mut buf = vec![0; length].into_boxed_slice();
    f.read_exact(&mut buf).unwrap();

    let s = from_utf8(&buf).unwrap();

    println!("read {}", s);

    s.to_string()
}
