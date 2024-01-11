pub trait List<T> {
    fn new(data: &[T]) -> Self;
    fn get(&self, index: usize) -> Option<&T>;
    fn get_all(&self) -> &[T];
    fn push(&mut self, item: T);
    fn len(&self) -> usize;
}
