extern crate explore;
extern crate hyper;

use hyper::server::{Handler, Server, Request, Response};
use std::{io, thread};
use std::io::Read;
use explore::core::Store;
use std::sync::{Arc, Mutex};

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

struct StoreWrap<T: Store> {
    store: T
}

impl Handler for StoreWrap<Arc<Mutex<Vec<String>>>> {
    fn handle(&self, mut req: Request, res: Response) {
        {
            println!("Received a request:");
            let method = &req.method;
            println!("Method: {}", method);
            let headers = &req.headers;
            println!("Headers: {}", headers);
            let uri = &req.uri;
            println!("uri: {}", uri);
        }

        let mut buffer = String::new();
        req.read_to_string(&mut buffer)
            .expect("failed to read to string");

        println!("Adding message: {}", buffer);
        let nr = self.store.clone().write(buffer);
        println!("Added message as number {}", nr);
    }
}

fn main() {
    let store = Arc::new(Mutex::new(vec!["ans".to_string(), "zwa".to_string(), "drei".to_string()]));

    let store_1 = store
        .clone();

    println!("Initializing server...");
    let server = Server::http("localhost:8080")
        .expect("Failed to initialize server");

    thread::spawn(move || {
        server
            .handle(StoreWrap { store: store_1 })
            .expect("Failed to handle");
    });

    println!("Server initialized");

    read(&store);
}


