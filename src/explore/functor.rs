pub trait StatFunctor<T, U> {
    type Rslt;
    fn map<F>(&self, f: F) -> Self::Rslt where F: Fn(&T) -> U;
}

use core::Opt;

impl<T, U> StatFunctor<T, U> for Opt<T> {
    type Rslt = Opt<U>;
    fn map<F>(&self, f: F) -> Self::Rslt where F: Fn(&T) -> U {
        match self {
            &Opt::None => Opt::None,
            &Opt::Some(ref t) => Opt::Some(f(t))
        }
    }
}
