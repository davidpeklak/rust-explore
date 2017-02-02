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
        ( $( $x:expr ),* )  => {{
            let mut tmp_list = List::Nil;
            $(
                tmp_list = List::Cons($x, Box::new(tmp_list)); 
            )*
            tmp_list            
        }}      
    }

    let n1 = List!(2, 3, 5, 8, 13, 21, 34, 55, 89, 144);

    println!("The i32 List: {}", n1);
}

