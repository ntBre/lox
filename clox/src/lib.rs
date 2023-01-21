pub mod chunk;

pub mod debug;

pub mod value {
    use std::ops::Index;

    pub type Value = f64;

    // this is probably not needed, but we'll see. keeping consistent with C
    // version for now. alternative would be constants: Vec<Value> directly on
    // Chunk
    pub struct ValueArray {
        values: Vec<Value>,
    }

    impl ValueArray {
        pub fn new() -> Self {
            Self { values: Vec::new() }
        }

        pub fn push(&mut self, value: Value) {
            self.values.push(value);
        }

        pub(crate) fn len(&self) -> usize {
            self.values.len()
        }
    }

    impl Default for ValueArray {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Index<usize> for ValueArray {
        type Output = Value;

        fn index(&self, index: usize) -> &Self::Output {
            &self.values[index]
        }
    }
}
