pub struct Stack<T> {
    items: Vec<T>
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            items: Vec::new()
        }
    }

    // Pushing onto the stack should be possible
    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    // Removing is only possible from the last item
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    // Check the last item, without removing it
    pub fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    // This returns if the stack has any items
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    // get the length of the stack, since items will be private
    pub fn length(&self) -> usize {
        self.items.len()
    }
}