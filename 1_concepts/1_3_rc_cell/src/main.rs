#![allow(dead_code, unused)]

use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

struct GlobalStack<T> {
    stack: Rc<RefCell<Vec<T>>>,
}

impl<T> GlobalStack<T> {
    fn new() -> Self {
        Self {
            stack: Rc::new(RefCell::new(vec![])),
        }
    }

    fn push(&self, data: T) {
        self.stack.borrow_mut().push(data);
    }

    fn pop(&self) -> Option<T> {
        self.stack.borrow_mut().pop()
    }

    fn peek(&self) -> Ref<Vec<T>> {
        let x = self.stack.as_ref().borrow();

        x
    }
}

impl<T> Clone for GlobalStack<T> {
    fn clone(&self) -> Self {
        Self {
            stack: Rc::clone(&self.stack),
        }
    }
}

fn main() {
    println!("Implement me!");
}

#[cfg(test)]
mod tests {
    use crate::GlobalStack;

    #[test]
    fn assert_cloning() {
        let stack = GlobalStack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        let cloned_stack = stack.clone();

        cloned_stack.push(4);

        assert_eq!(vec![1, 2, 3, 4], *(stack.peek()));
        assert_eq!(*(cloned_stack.peek()), *(stack.peek()));
    }
}
