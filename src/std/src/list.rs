//! Standard library: List<T> for Gigli

#[derive(Debug, Clone)]
pub struct List<T> {
    data: Vec<T>,
}

impl<T> List<T> {
    /// Creates a new empty list.
    pub fn new() -> Self {
        List { data: Vec::new() }
    }

    /// Adds an element to the end of the list.
    pub fn push(&mut self, value: T) {
        self.data.push(value);
    }

    /// Removes and returns the last element, if any.
    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    /// Returns the number of elements in the list.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns true if the list is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Gets a reference to the element at the given index.
    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    /// Gets a mutable reference to the element at the given index.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.data.get_mut(index)
    }

    /// Sets the value at the given index.
    pub fn set(&mut self, index: usize, value: T) {
        if index < self.data.len() {
            self.data[index] = value;
        }
    }

    /// Returns an iterator over the list.
    pub fn iter(&self) -> std::slice::Iter<T> {
        self.data.iter()
    }

    /// Returns a mutable iterator over the list.
    pub fn iter_mut(&mut self) -> std::slice::IterMut<T> {
        self.data.iter_mut()
    }

    /// Filters the list, returning a new List with elements that match the predicate.
    pub fn filter<F>(&self, mut f: F) -> List<T>
    where
        T: Clone,
        F: FnMut(&T) -> bool,
    {
        List {
            data: self.data.iter().cloned().filter(|x| f(x)).collect(),
        }
    }

    /// Maps the list, returning a new List with the results.
    pub fn map<U, F>(&self, mut f: F) -> List<U>
    where
        F: FnMut(&T) -> U,
    {
        List {
            data: self.data.iter().map(|x| f(x)).collect(),
        }
    }
}
