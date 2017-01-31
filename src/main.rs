use std::fmt;

fn main() {

    enum List <'a, T: 'a> {
        Nil,
        Cons { head: T, tail: &'a List<'a, T> } 
    }

    impl<'a, T: fmt::Display> fmt::Display for List<'a, T> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                &List::Nil => write!(f, "Nil"),
                &List::Cons{ head: ref h, tail: t } => {
                    write!(f, "{} :: ", h);
                    t.fmt(f)
                }
            } 
        }
    }    

    let n = List::Nil;
    let n1 = List::Cons{head: 2, tail: &n};

    println!("The i32 List: {}", n1);

    let s = List::Nil;
    let s1 = List::Cons{head: "Hallo", tail: &s};

    println!("The str List: {}", s1);
}

