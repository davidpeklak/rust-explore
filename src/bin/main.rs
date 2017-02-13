#[macro_use]
extern crate explore;
extern crate byteorder;

use std::io;
use std::fs::File;

fn main() {
    {
        let mut file = File::create("target/foo.txt")
            .expect("Could not open file");

        loop {
            println!("Enter a message");

            let mut input = String::new();

            io::stdin().read_line(&mut input)
                .expect("Failed to read line");

            let trimmed_input = input.trim();

            if trimmed_input.len() == 0 {
                break;
            }

            write_to_file(&mut file, &trimmed_input)
        }
    }
    {
        let mut file = File::open("target/foo.txt").unwrap();

        loop {
            let msg = read_from_file(&mut file);
            println!("{}", msg);
        }
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

    file.write_all(&ba)
        .expect("Could not right message length to file");
    file.write_all(msg.as_bytes())
        .expect("Could not right message to file");
}

fn read_from_file(file: &mut File) -> String {
    use std::io::Read;
    use std::str::from_utf8;
    use byteorder::{ByteOrder, BigEndian};

    let mut ba = [0u8; 4];
    file.read_exact(&mut ba)
        .expect("Could not read message length from file");

    let length = BigEndian::read_u32(&mut ba) as usize;
    println!("read length: {}", length);

    let mut buf = vec![0; length].into_boxed_slice();
    file.read_exact(&mut buf)
        .expect("Could not read message from file");

    let s = from_utf8(&buf).unwrap();

    println!("read {}", s);

    s.to_string()
}
