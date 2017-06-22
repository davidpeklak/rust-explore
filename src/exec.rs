use super::future::FutureImpl;

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
