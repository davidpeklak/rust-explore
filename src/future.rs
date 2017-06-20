use std::marker::Sized;

pub trait Future
    where Self: Sized {
    type T;
    fn map<U, M>(self, mf: M) -> Map<Self, M>
        where M: Fn(Self::T) -> U {
        Map { fut: self, mf: mf }
    }
}

pub struct FutureImpl<F> {
    pub fun: F
}

pub struct Map<F, M> {
    pub fut: F,
    pub mf: M
}

impl<T, F> Future for FutureImpl<F>
    where F: Fn() -> T {
    type T = T;
}

impl<T, F, M, U> Future for Map<F, M>
    where F: Future<T=T>,
          M: Fn(T) -> U {
    type T = U;
}
