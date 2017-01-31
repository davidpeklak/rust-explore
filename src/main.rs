use std::fmt;

fn main() {

    enum List <'a> {
        Nil,
        Cons { head: i32, tail: &'a List<'a> } 
    }

    fn prnt(l: &List) {
        match l {
            &List::Nil => println!("Nil"),
            &List::Cons { head: h, tail: t } =>  { 
               println!("{}", h);
               prnt(t)
            }
        }
    }

    impl<'a> fmt::Display for List<'a> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                &List::Nil => write!(f, "Nil"),
                &List::Cons{ head: h, tail: t } => {
                    write!(f, "{} :: ", h);
                    t.fmt(f)
                }
            } 
        }
    }    

    let n = List::Nil;
    let n1 = List::Cons{head: 2, tail: &n};

    // prnt(&n1);
    println!("The winner is: {}", n1);
}

