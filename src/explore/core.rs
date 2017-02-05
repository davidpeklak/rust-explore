use std::fmt;

pub trait Foo {
    fn to_a_string(&self) -> String;
}

pub enum Opt<T> {
    None,
    Some(T)
}

impl<T: fmt::Display> fmt::Display for Opt<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Opt::None => write!(f, "None"),
            &Opt::Some(ref t) => write!(f, "Some({})", t)
        }
    }
}

impl<T: fmt::Display> Foo for Opt<T> {
    fn to_a_string(&self) -> String {
        match self {
            &Opt::None => "None".to_string(),
            &Opt::Some(ref t) => format!("Some({})", t)
        }
    }
}

pub enum List <T> {
    Nil,
    Cons(T, Box<List<T>>)
}

impl<T: fmt::Display> fmt::Display for List<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &List::Nil => write!(f, "Nil"),
            &List::Cons(ref h, ref bt ) => {
                write!(f, "{} :: ", h)
                    .and(bt.fmt(f))
            }
        }
    }
}

impl<T: fmt::Display> Foo for List<T> {
    fn to_a_string(&self) -> String {
        match self {
            &List::Nil => "Nil".to_string(),
            &List::Cons(ref h, ref bt ) => {
                format!("{} :: {}", h, bt.to_a_string())
            }
        }
    }
}