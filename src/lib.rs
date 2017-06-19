use std::marker::Sized;
use std::marker::PhantomData;

pub trait Future<T>
where Self: Sized {
    fn map<U, M>(self, mf: M) -> Map<T, Self, M>
        where M: Fn(T) -> U;
}

pub struct FutureImpl<T, F>
    where F: Fn() -> T {
    pub fun: F
}

pub struct Map<T, F, M>
    where F: Future<T> {
    pub fut: F,
    pub mf: M,
    phantom_t: PhantomData<T>
}

impl<T, F> Future<T> for FutureImpl<T, F>
    where F: Fn() -> T {
    fn map<U, M>(self, mf: M) -> Map<T, Self, M>
        where M: Fn(T) -> U {
        Map { fut: self, mf: mf, phantom_t: PhantomData }
    }
}

impl<T, F, M, U> Future<U> for Map<T, F, M>
    where F: Future<T>,
          M: Fn(T) -> U {
    fn map<V, N>(self, mf: N) -> Map<U, Self, N>
        where N: Fn(U) -> V {
        Map { fut: self, mf: mf, phantom_t: PhantomData }
    }
}
