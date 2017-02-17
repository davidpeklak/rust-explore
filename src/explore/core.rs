use std::sync::{Arc, Mutex};

pub trait Store {
    fn write(&mut self, msg: String) -> usize;
    fn max(&self) -> usize;
    fn read(&self, i: usize) -> Option<String>;
}

impl Store {
    pub fn new() -> Box<Store> {
        Box::new(Vec::new())
    }
}

impl Store for Vec<String> {
    fn write(&mut self, msg: String) -> usize {
        self.push(msg);
        self.len()
    }

    fn max(&self) -> usize {
        self.len()
    }

    fn read(&self, i: usize) -> Option<String> {
        if i < self.len() {
            Some(self[i].clone())
        }
        else {
            None
        }
    }
}

impl Store for Arc<Mutex<Vec<String>>> {
    fn write(&mut self, msg: String) -> usize {
        let mut vec = self
            .lock()
            .unwrap();
        vec.push(msg);
        vec.len()
    }

    fn max(&self) -> usize {
        let vec = self
            .lock()
            .unwrap();
        vec.len()
    }

    fn read(&self, i: usize) -> Option<String> {
        let vec = self
            .lock()
            .unwrap();

        if i < vec.len() {
            Some(vec[i].clone())
        }
            else {
                None
            }
    }
}