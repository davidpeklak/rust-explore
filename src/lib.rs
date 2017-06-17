pub trait Future<T> {
    fn map<U, M>(self, mf: M) -> Map<Self, M>
        where M: Fn(T) -> U;
}

pub struct FutureImpl<T, F>
    where F: Fn() -> T {
    pub fun: F
}

pub struct Map<F, M> {
    pub fut: F,
    pub mf: M
}

impl<T, F> FutureImpl<T, F>
    where F: Fn() -> T {
    pub fn map<U, M>(self, mf: M) -> Map<Self, M>
        where M: Fn(T) -> U {
        Map { fut: self, mf: mf }
    }
}

impl<T, F, M, U> Map<FutureImpl<T, F>, M>
    where F: Fn() -> T,
          M: Fn(T) -> U {
    pub fn map<V, N>(self, mf: N) -> Map<Self, N>
        where N: Fn(U) -> V {
        Map { fut: self, mf: mf }
    }
}
