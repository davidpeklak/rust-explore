use std::fmt;

fn main() {

   trait Foo {
       fn to_a_string(&self) -> String;
   }

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

    impl<T: fmt::Display> Foo for Opt<T> {
        fn to_a_string(&self) -> String {
            match self {
                &Opt::None => "None".to_string(),
                &Opt::Some(ref t) => format!("Some({})", t) 
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
         

    macro_rules! List {
        ( $( $x:expr ),* )  => {{
            let mut tmp_list = List::Nil;
            $(
                tmp_list = List::Cons($x, Box::new(tmp_list)); 
            )*
            tmp_list            
        }}      
    }

    fn stat_dispatch<T: Foo>(t: T) {
        println!("{}", t.to_a_string())
    }

    let n1 = List!(2, 3, 5, 8, 13, 21, 34, 55, 89, 144);

    stat_dispatch(n1);

    let o1 = Opt::Some(3);

    stat_dispatch(o1);

    let o2: Opt<i32>  = Opt::None;

    stat_dispatch(o2);


}

