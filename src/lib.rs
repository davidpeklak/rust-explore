
pub struct Future<T, F>
    where F: Fn() -> T {
    pub fun: F
}

pub struct Map<M> {
    pub mf: M
}

impl<T, F> Future<T, F>
    where F: Fn() -> T {
    pub fn map<U, M>(&self, mf: M) -> Map<M>
        where M: Fn(T) -> U {
        Map { mf: mf }
    }
}
