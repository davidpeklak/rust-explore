---
layout: post
title:  "Map the future"
date:   2017-06-15
categories: jekyll update
---
As a next step, let's try to implement a map function of our `Future` structure.
We want to use it like this:
```rust
let fut = fut.map(|x| x + 2);
```
Let's try to implement this function. Let's first look into the parameter of the function,
later at its return type.
The parameter of our function is a mapping function that takes a value of type `T` of the future, and returns a value of another type (let's call it `U` in the code). We want to we have two choices:
[static or dynamic dispatch|https://doc.rust-lang.org/1.0.0-beta/book/static-and-dynamic-dispatch.html].
Dynamic dispatch would look like this:
```rust
impl<T, F> Future<T, F>
    where F: Fn() -> T {
    pub fn map<U>(&self, mf: Fn(T) -> U) -> ... {
        ...
    }
}
```
Static dispatch would look like this:
```rust
impl<T, F> Future<T, F>
    where F: Fn() -> T {
    pub fn map<U, M>(&self, mf: M) -> ... 
    where M: Fn(T) -> U {
        ...
    }
}
```
What is the difference? In the static dispatch version, we know the concrete type of the
mapping function at compile time. Since we probably want to store this function in some
structure that we want to be able to allocate on the stack, knowing the concrete type
(and, specifically, its size) is a necessity. So let's go for the static dispatch version.
What shall the return type of our function be? Well, as just said, some structure that 
holds the mapping function. And also the future that we want to map. Let's call it `Map`,
and start off with an empty structure for now.
```rust
pub struct Map {}
```
Nice, [this compiles](https://github.com/davidpeklak/rust-explore/commit/f9417f9e8068d16aa5c82548d2076887cda8b9f7).
We get two warnings though. First, the result of our mapping in the main function is
unused. Fine. Second, the mapping function that we pass to the `map` funtion is unused.
If we want to do anything at all with this mapping function, we better use it, and put it 
into the `Map` structure as mentioned before.
```rust
pub struct Map<M> {
    pub mf: M
}
```
Good, [like this](https://github.com/davidpeklak/rust-explore/commit/003233de42153136dd56e698948b6c2b4748dc2a)
we get rid of one compiler warning.
Now, it might seem a good idea to restrict the type parameter `M` in the `Map` structure:
```rust
pub struct Map<T, U, M>
    where M: Fn(T) -> U {
    pub mf: M
}
```
But the compiler [does not like this](https://github.com/davidpeklak/rust-explore/compare/003233d...ef1341f).
It tells us that parameters `T` and `U` are never used, and suggests to use `PhantomData` of
these types in our structure. Let's not do this. We will see later why this restriction is
not really necessary anyway. But one thing that we should do is to also store the future
that we want to map in the `Map` structure:
```rust
pub struct Map<F, M> {
    pub fut: F,
    pub mf: M
}
```
Again, we don't restrict the type `F` in the structure. Let's try to [compile this](https://github.com/davidpeklak/rust-explore/commit/957a39b2413e3e7853f5f603e015775178812fe3).
Omg, doesn't compile! Since we store the actual future in the `Map`structure, and not just a
reference, we need to change the type of the future in the `map` function from `&self` to
`self`:
```rust
pub fn map<U, M>(self, mf: M) -> Map<Self, M>
    where M: Fn(T) -> U {
    Map { fut: self, mf: mf }
}
```
[Looks better!](https://github.com/davidpeklak/rust-explore/commit/4471a00631568695faae4db6c94a90ca0444bee7)






