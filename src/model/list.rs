pub(crate) trait List<T> {
    fn new() -> Self;
    fn get(&self, index: usize) -> Option<&T>;
    fn push(&mut self, item: T);
    fn len(&self) -> usize;
    fn iter(&self) -> std::slice::Iter<'_, T>;
}
