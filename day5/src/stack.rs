pub struct Stack<T>(Vec<T>);

impl<T> Stack<T> {

    pub fn new() -> Self {
        Self(Vec::<T>::new())
    }

    pub fn push(&mut self, val: T) {
        self.0.push(val);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }

    pub fn height(&self) -> usize {
        self.0.len()
    }

    pub fn top(&self) -> Option<&T> {
        self.0.last()
    }
}
