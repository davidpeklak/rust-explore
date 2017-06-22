pub trait Execute {
    type T;
    fn run_sync(&self) -> Self::T;
}
