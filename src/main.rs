extern crate futexp;

use futexp::future::*;
use futexp::exec::Execute;

fn main() {
    let fut = future(|| 3);
    let fut = fut.map(|x| x + 2);
    let fut = fut.map(|x| x * 2);
    let fut = fut.map(|x| x - 3);
    let fut = fut.map(|x| { println!("x is {}", x) });

    fut.run_sync();
    fut.run_sync();
    fut.run_sync();
}
