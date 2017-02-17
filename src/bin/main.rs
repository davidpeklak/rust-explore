extern crate explore;

use std::{io, thread};
use explore::core::Store;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn read(s: &Store) {
    loop {
        println!("Enter 'max' to get maximum index, enter an index to get the referenced message");

        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        let trimmed_input = input.trim();

        if trimmed_input.len() == 0 {
            break;
        }

        if trimmed_input == "max" {
            println!("max is {}", s.max());
        } else {
            let i: usize = trimmed_input
                .parse()
                .expect("expected a number");

            match s.read(i) {
                Some(s) => println!("{}: {}", i, s),
                None => println!("{}: None", i)
            }
        }
    }
}

fn grow(s: & mut Store) {
    loop {
        s.write("vier".to_string());
        thread::sleep(Duration::from_millis(2000));
    }
}

fn main() {
    let store = Arc::new(Mutex::new(vec!["ans".to_string(), "zwa".to_string(), "drei".to_string()]));

    let mut store_1 = store
        .clone();

    thread::spawn(move || {
        grow(&mut store_1);
    });

    read(&store);

}


