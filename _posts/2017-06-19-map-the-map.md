---
layout: post
title:  "Map the Map"
date:   2017-06-19
categories: jekyll update
---
In the [last post](https://davidpeklak.github.io/rust-explore/jekyll/update/2017/06/15/map-the-future.html) we mapped a future, today, let's map a `Map`. We take the same
approach:
```rust
impl<T, F, M, U> Map<Future<T, F>, M>
    where F: Fn() -> T,
          M: Fn(T) -> U {
    pub fn map<V, N>(self, mf: N) -> Map<Self, N>
        where N: Fn(U) -> V {
        Map { fut: self, mf: mf }
    }
}
```
[This compiles](https://github.com/davidpeklak/rust-explore/commit/e411ef0ff4854b40dffd831549b2274b994a9765),
and two things can be observed here:
1. The implementation is a good place to place restrictions on type parameters
(`where F: Fn() -> T`, `M: Fn(T) -> U). I have mentioned in the [last post](https://davidpeklak.github.io/rust-explore/jekyll/update/2017/06/15/map-the-future.html) that the 
`Map` structure might not be an ideal place for this. The implementation seems to be a
better place.
2. This does not scale! What happens if we want to map our mapped map?
[It's getting ugly](https://github.com/davidpeklak/rust-explore/commit/3fbc2b7ba928d3282f544cf0c238531b577fca34).
In order to avoid this uglyness, let's factor our a trait! It's probably good to call this
trait `Future`, so we start by renaming our current `Future` structure to `FutureImpl`, like
[here](https://github.com/davidpeklak/rust-explore/commit/c5ec1e6aa4da3bab40f0b9bec520d7f3d1e593e1).
Now, let's try to define a trait:
```rust
pub trait Future<T> {
    fn map<U, M>(self, mf: M) -> Map<Self, M>
        where M: Fn(T) -> U;
}
```
The [compiler complains here](https://github.com/davidpeklak/rust-explore/commit/974d5d817cd088ef96c90c47e4321e0001e21b99),
that "`std::marker::Sized` is not implemented for `Self`", and suggests to 
"consider adding a `where Self: std::marker::Sized` bound". Rememer that we want to
allocate the `Map` structure on the stack, therefore its size must be known. This means that
the size of all its parts must also be known, and the type `Self` is part of the `Map`
structure that we return. So let's do what the compiler suggests:
```rust
pub trait Future<T>
where Self: Sized {
    fn map<U, M>(self, mf: M) -> Map<Self, M>
        where M: Fn(T) -> U;
}
```
This makes the compiler [happy](https://github.com/davidpeklak/rust-explore/commit/544b053bb0130e22c2e2f448c1ca47799bdf9bed).
But what does that mean? How can a trait be sized? It does not actually mean that the trait
is sized, it means that all types that implement the trait must be sized.
Now, we can implement `Future` for `FutureImpl`:
```rust
impl<T, F> Future<T> for FutureImpl<T, F>
    where F: Fn() -> T {
    fn map<U, M>(self, mf: M) -> Map<Self, M>
        where M: Fn(T) -> U {
        Map { fut: self, mf: mf }
    }
}
```
and for `Map`:
```rust
impl<T, F, M, U> Future<U> for Map<FutureImpl<T, F>, M>
    where F: Fn() -> T,
          M: Fn(T) -> U {
    fn map<V, N>(self, mf: N) -> Map<Self, N>
        where N: Fn(U) -> V {
        Map { fut: self, mf: mf }
    }
}
```
[Nice](https://github.com/davidpeklak/rust-explore/commit/eb77bc8670a97f7bd238b9a78809dd43fd108a3d).
But does this scale, that is, does it allow us to map a mapped `Map`?
[Computer says no!](https://github.com/davidpeklak/rust-explore/commit/3496d5f90af445f4bf9097f28a0f886f9279a7e5)
Let's use the `Future` trait instead of the `FutureImpl` in the implementation:
```rust
impl<T, F, M, U> Future<U> for Map<F, M>
    where F: Future<T>,
          M: Fn(T) -> U {
    fn map<V, N>(self, mf: N) -> Map<Self, N>
        where N: Fn(U) -> V {
        Map { fut: self, mf: mf }
    }
}
```
Looks good, but [does not compile](https://github.com/davidpeklak/rust-explore/commit/8bc460d55e2332e5202ae24ab851dc4c20a734fd).
The compiler says that "the type parameter `T` is not constrained". Hmmm. One way
to make it constraint would be to use it in the `Map` structure. But that would
force us to introduce a `PhantomData` field of type `T` in the `Map` structure and
that is [kind of ugly](https://github.com/davidpeklak/rust-explore/commit/3be40663b49e23e86c43441604b89b5415dbdbcc).
We can also overcome the problem by not having to mention `T` when we say
`where F: Future<T>`. How can we to that? By making `T` an [associated type](https://doc.rust-lang.org/beta/book/first-edition/associated-types.html)
of `Future`!
```rust
pub trait Future
    where Self: Sized {
    type T;
    fn map<U, M>(self, mf: M) -> Map<Self, M>
        where M: Fn(T) -> U;
}
```
[This compiles](https://github.com/davidpeklak/rust-explore/commit/56a65650754d78c8b3d2c6ebca6af8c2313e8568),
and it actually looks a bit tidier! We even observe that now the implementations
of the map function look exatly the same in the implementation for `FutureImpl`
and `Map`:
```rust
fn map<U, M>(self, mf: M) -> Map<Self, M>
    where M: Fn(T) -> U {
    Map { fut: self, mf: mf }
}
```
so we can extract the implementation to the `Future` trait.
[Nice and concise](https://github.com/davidpeklak/rust-explore/commit/fc21c77754dbeb763b4bf90c264b2645081a8378)!
