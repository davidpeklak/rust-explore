
pub struct Future<T, F>
    where F: Fn() -> T {
    pub fun: F
}
