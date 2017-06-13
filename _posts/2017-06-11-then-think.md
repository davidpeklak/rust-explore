---
layout: post
title:  "Then think"
date:   2017-06-11
categories: jekyll update
---
Now, let's think about what we are actually trying to achive. Since we are trying to build a
library for futures, we should define what we want a future to be:
* A future shall be something that eventually produces a value.
* A future can have effects, like IO (reading and/or writing from/to files or sockets), or
change internal state of the program.
* Futures shall be easily composable: in funcional-programming lingo they should be functors,
applicative functors, and if we get there, monads. In other words, they should implement a
map function, a function that joins two futures (that might run parallel) together, and a
flat_map function.
* It shall be possible to allocate futures on the stack, for performance reasons. Even though
"early optimization is the source of all evil", this choice will probalby have a major impact
on the disign and therefore we should take it into account from the beginning.
* Futures should be executable more than once. So once we have created a future we want to
be able to execute it several times, possibly concurrently.

Well, that's already quite a bunch of requirements, let's start with the first one: "a future
is something that eventually produces a value." Fine, so this sounds like a structure that
wraps around a closure. Should be easy enough.
```rust
pub struct Future<T> {
    pub fun: Closure() -> T
}
```
`Closure` doesn't actually exist in the standard library. We have three choices here:
`FnOnce`, `FnMut`and `Fn`. There is an excellent post about which one to chose when by
Houn Wilson [here](http://huonw.github.io/blog/2015/05/finding-closure-in-rust/), so
let's go ahead and read it!
Good, so since one of our requirements is that futures should be executable many times,
possibly concurrently, let's chose `Fn`.
```rust
pub struct Future<T> {
    pub fun: Fn() -> T
}
```
Yippie, [that compiles](https://github.com/davidpeklak/rust-explore/commit/9bd46040fe46fc01ab75f1c0030ac05a8842240a).
Let's try to use it:
```rust
fn main() {
    let fut = Future { fun: || 3 };
}
```
Bummer, three error messages when we try to [compile this](https://github.com/davidpeklak/rust-explore/commit/1ec4a46fee41b08a71eece6603f35878f80cbc12)!
The compiler tells us that" within `futexp::Future<{integer}>`,
the trait `std::marker::Sized` is not implemented for `std::ops::Fn() -> {integer} + 'static`"
Why does it have to be `Sized`? Because we just tried to allocate it on the stack, and for
anything that goes onto the stack, the size must be known at compile time. The thing is,
the size of our closure `|| 3` _is_ actually known at compile, but we somehow lose this
information in the way that we defined our `Future` structure. We need to use the concrete
type of the closure, not just `Fn`.
```rust
pub struct Future<T, F>
    where F: Fn() -> T {
    pub fun: F
}

```
Now we can allocate the structure on the stack, and our [code compiles](https://github.com/davidpeklak/rust-explore/commit/8b4c4849e1ece978f05b1455e73acd8d8a5a62cf).
We get a warning about `fut` not being used, but this is expected.
