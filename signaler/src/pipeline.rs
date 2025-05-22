// Like A -> B -> C kind of thing

pub struct Pipeline<T> {
    pipes: Vec<Pipe<T, T>>,
}

pub struct Pipe<TInput, TOutput> {}

impl<TInput> Pipeline<TInput> {
    pub fn new() -> Self {}

    pub fn map<F, TOutput>(self, f: F) -> Pipe<TInput, TOutput>
    where
        F: Fn(TInput) -> TOutput,
    {
        Pipe {}
    }
}
