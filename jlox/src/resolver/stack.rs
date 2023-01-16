use std::ops::Index;

#[derive(Debug)]
pub(crate) struct Stack<T> {
    pub(crate) data: Vec<T>,
}

impl<T> Stack<T> {
    pub(crate) fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub(crate) fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub(crate) fn peek(&mut self) -> &mut T {
        self.data.last_mut().unwrap()
    }

    pub(crate) fn len(&self) -> usize {
        self.data.len()
    }
}

impl<T> Index<usize> for Stack<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> Stack<T>
where
    T: Default,
{
    pub(crate) fn push_default(&mut self) {
        self.data.push(T::default())
    }
}
