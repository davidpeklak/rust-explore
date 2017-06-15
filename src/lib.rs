
pub struct Future<T, F>
    where F: Fn() -> T {
    pub fun: F
}

pub struct Map {}

impl<T, F> Future<T, F>
    where F: Fn() -> T {
    pub fn map<U, M>(&self, mf: M) -> Map
        where M: Fn(T) -> U {
        Map {}
    }
}
