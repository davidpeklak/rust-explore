
pub struct Future<T, F>
    where F: Fn() -> T {
    pub fun: F
}

pub struct Map<F, M> {
    pub fut: F,
    pub mf: M
}

impl<T, F> Future<T, F>
    where F: Fn() -> T {
    pub fn map<U, M>(self, mf: M) -> Map<Self, M>
        where M: Fn(T) -> U {
        Map { fut: self, mf: mf }
    }
}
