#[macro_use]
extern crate explore;
extern crate byteorder;

use explore::core::{Foo, Opt, List};
use explore::functor::StatFunctor;

fn main() {


    let n1 = List!(2, 3, 5, 8, 13, 21, 34, 55, 89, 144);

    stat_dispatch(&n1);
    dyn_dispatch(&n1);
    write_to_file(&n1);
    read_from_file();

    let o1 = Opt::Some(3);

    stat_dispatch(&o1);
    dyn_dispatch(&o1);

    let o2: Opt<i32>  = Opt::None;

    stat_dispatch(&o2);
    dyn_dispatch(&o2);

    let o3: Opt<i32> = o1.map(|x| x + 2);

    stat_dispatch(&o3);
    dyn_dispatch(&o3);



}

fn stat_dispatch<T: Foo>(t: &T) {
    println!("stat: {}", t.to_a_string())
}

fn dyn_dispatch(t: &Foo) {
    println!("dyn: {}", t.to_a_string())
}

fn write_to_file<T: Foo>(t: &T) {
    use std::io::prelude::*;
    use std::fs::File;
    use byteorder::{ByteOrder, BigEndian};

    let bytes = t.to_a_string();
    let length = bytes.len() as u32;
    let mut ba = [0u8; 4];
    BigEndian::write_u32(&mut ba, length);

    let mut f = File::create("target/foo.txt").unwrap();

    println!("write length: {}", length);

    f.write_all(&ba).unwrap();
    f.write_all(t.to_a_string().as_bytes()).unwrap();
}

fn read_from_file()  {
    use std::fs::File;
    use std::io::Read;
    use byteorder::{ByteOrder, BigEndian};

    let mut f = File::open("target/foo.txt").unwrap();

    let mut ba = [0u8; 4];
    f.read_exact(&mut ba).unwrap();

    let length = BigEndian::read_u32(&mut ba) as usize;

    println!("read length: {}", length);
}
