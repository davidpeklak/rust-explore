use std::fmt;

fn main() {

    enum Opt<T> {
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

    enum List <T> {
        Nil,
        Cons(T, Box<List<T>>)
    }

    impl<T: fmt::Display> fmt::Display for List<T> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                &List::Nil => write!(f, "Nil"),
                &List::Cons(ref h, ref bt ) => {
                    write!(f, "{} :: ", h);
                    bt.fmt(f)
                }
            } 
        }
    }    

    macro_rules! List {
        ($e1:expr, $e2:expr) => (List::Cons($e1, Box::new(List::Cons($e2, Box::new(List::Nil)))));      
    }


    let n1 = List!(3, 2);

    println!("The i32 List: {}", n1);

    let s = Box::new(List::Nil);
    let s1 = List::Cons("Hallo", s);;

    println!("The str List: {}", s1);
}

