use std::marker::Sized;

pub trait Future<T>
where Self: Sized {
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

impl<T, F> Future<T> for FutureImpl<T, F>
    where F: Fn() -> T {
    fn map<U, M>(self, mf: M) -> Map<Self, M>
        where M: Fn(T) -> U {
        Map { fut: self, mf: mf }
    }
}

impl<T, F, M, U> Future<U> for Map<FutureImpl<T, F>, M>
    where F: Fn() -> T,
          M: Fn(T) -> U {
    fn map<V, N>(self, mf: N) -> Map<Self, N>
        where N: Fn(U) -> V {
        Map { fut: self, mf: mf }
    }
}
