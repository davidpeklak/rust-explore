extern crate explore;

use explore::core::{Foo, Opt, List};

fn main() {

    macro_rules! List {
        ( $( $x:expr ),* )  => {{
            let mut tmp_list = List::Nil;
            $(
                tmp_list = List::Cons($x, Box::new(tmp_list)); 
            )*
            tmp_list            
        }}      
    }

    fn stat_dispatch<T: Foo>(t: &T) {
        println!("stat: {}", t.to_a_string())
    }

    fn dyn_dispatch(t: &Foo) {
        println!("dyn: {}", t.to_a_string())
    }

    let n1 = List!(2, 3, 5, 8, 13, 21, 34, 55, 89, 144);

    stat_dispatch(&n1);
    dyn_dispatch(&n1);

    let o1 = Opt::Some(3);

    stat_dispatch(&o1);
    dyn_dispatch(&o1);

    let o2: Opt<i32>  = Opt::None;

    stat_dispatch(&o2);
    dyn_dispatch(&o2);

}
