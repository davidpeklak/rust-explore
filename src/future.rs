use std::marker::Sized;

/// a trait that represents a computation that eventually will yield a value of type `T`
pub trait Future
    where Self: Sized {
    /// the type of the value that the future will eventually yield
    type T;

    /// maps the `Future` with the function `fun`
    ///
    /// # Examples
    ///
    /// ```
    /// let f = future(|| 3);
    /// let f = f.map(|x| x + 2);
    /// ```
    fn map<U, M>(self, mf: M) -> Map<Self, M>
        where M: Fn(Self::T) -> U {
        Map { fut: self, mf: mf }
    }
}

/// creates a `Future` from the function `fun`
///
/// # Examples
///
/// ```
/// let f = future(|| 3);
/// ```
pub fn future<T, F>(fun: F) -> FutureImpl<F>
    where F: Fn() -> T {
    FutureImpl { fun: fun }
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
