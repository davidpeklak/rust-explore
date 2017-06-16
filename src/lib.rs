
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

impl<T, F, M, U> Map<Future<T, F>, M>
    where F: Fn() -> T,
          M: Fn(T) -> U {
    pub fn map<V, N>(self, mf: N) -> Map<Self, N>
        where N: Fn(U) -> V {
        Map { fut: self, mf: mf }
    }
}

impl<T, F, M, U, V, N> Map<Map<Future<T, F>, M>, N>
    where F: Fn() -> T,
          M: Fn(T) -> U,
          N: Fn(U) -> V {
    pub fn map<W, O>(self, mf: O) -> Map<Self, O>
        where O: Fn(V) -> W {
        Map { fut: self, mf: mf }
    }
}
