extern crate futexp;

use futexp::*;

fn main() {
    let fut = FutureImpl { fun: || 3 };
    let fut = fut.map(|x| x + 2);
    let fut = fut.map(|x| x * 2);
    let fut = fut.map(|x| x - 3);
}
