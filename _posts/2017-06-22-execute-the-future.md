---
layout: post
title:  "Execute the Future"
date:   2017-06-22
categories: jekyll update
---
In this post, we will attempt to execute the futures that we have constructed in the [last post](https://davidpeklak.github.io/rust-explore/jekyll/update/2017/06/19/map-the-map.html).
To get prepared, let's re-organize the code a bit, and put the code that we have in a
[dedicated module](https://github.com/davidpeklak/rust-explore/commit/d09fe696213addf410e9d36f4f28ec08ed1d1260)
named `future`, and let's add
[some documentation](https://github.com/davidpeklak/rust-explore/commit/a1eec9157b09a57a31e04d8e202a34a4f101200b).
Our approach will be to define an `Execute` trait and then implement that trait for the
structures we have defined so far, that is `FutureImpl` and `Map`. This approach is flexible,
because we can define different traits to execute our futures, with different behaviours.
We start off with a very simple one, executing the futures syncronously on the calling thread.
```rust
pub trait Execute {
    type T;
    fn run_sync(&self) -> Self::T;
}
```
[Here](https://github.com/davidpeklak/rust-explore/commit/982df169b0c73653f435f0d8275a2abd1e60a847),
we are saying: to implement this trait, you must define a type `T`, and a function `run_sync`
that takes an immutable reference to `self` and returns a `T`. Good! Let's try and implement
this for `FutureImpl`!
`FutureImpl` takes a type parameter `F`, and in our implementation, we will have to specify
the type assiciated type `T` of the `Execute` trait, so the implementation will look
something like:
```rust
impl<T, F> Execute for FutureImpl<F> {
    type T = T;

    ...
}
```
Also, we need to restrict `F`the same way we restricted it on the `Future` implementation for
`FutureImpl`:
```rust
impl<T, F> Execute for FutureImpl<F> 
    where F: Fn() -> T {
    type T = T;

    ...
}
```
Almost there, now we only need to implement the `run_sync` function, and all it has to do
is call the `fun` of the `FutureImpl`:
```rust
impl<T, F> Execute for FutureImpl<F>
    where F: Fn() -> T {
    type T = T;

    fn run_sync(&self) -> Self::T {
        (self.fun)()
    }
}
```
Mind the brackets that are needed around `(self.fun)`. Remember that in the [second post](https://davidpeklak.github.io/rust-explore/jekyll/update/2017/06/11/then-think.html)
we required that futures, once constructed, can be eecuted more than once. Is that
possible with our current implementation?
[Yes yes yes!](https://github.com/davidpeklak/rust-explore/commit/7f8515542169c71584b13cb92befffe7a2bda404), because we are not taking ownership of
`self` when we `run_sync`.
Next, let's also implement `Execute` for `Map`. This will look like this:
```rust
impl<T, F, M, U> Execute for Map<F, M>
    where F: Execute<T=T>,
          M: Fn(T) -> U {
    type T = U;

    fn run_sync(&self) -> Self::T {
        unimplemented!()
    }
}
```
Note that here we don't actually care if the type `F` is a `Future`. We just need it to be
an `Execute`. How do we implement `run_sync`? We first call `run_sync`on the `fut` of `Map`,
then call `mf` on the result:
```rust
impl<T, F, M, U> Execute for Map<F, M>
    where F: Execute<T=T>,
          M: Fn(T) -> U {
    type T = U;

    fn run_sync(&self) -> Self::T {
        (self.mf)(self.fut.run_sync())
    }
}
```
[Voila](https://github.com/davidpeklak/rust-explore/commit/58b949db5b4270c41a60d95ecb91b63a22096ce2)!
