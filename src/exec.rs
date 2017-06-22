use super::future::{FutureImpl, Map};

pub trait Execute {
    type T;
    fn run_sync(&self) -> Self::T;
}

impl<T, F> Execute for FutureImpl<F>
    where F: Fn() -> T {
    type T = T;

    fn run_sync(&self) -> Self::T {
        (self.fun)()
    }
}

impl<T, F, M, U> Execute for Map<F, M>
    where F: Execute<T=T>,
          M: Fn(T) -> U {
    type T = U;

    fn run_sync(&self) -> Self::T {
        (self.mf)(self.fut.run_sync())
    }
}
