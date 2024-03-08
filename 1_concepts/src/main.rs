use std::ops::Deref;
use std::sync::{Arc, RwLock};
use std::thread;
use std::thread::{sleep};
use std::time::Duration;

use arc_swap::ArcSwapOption;

type ListRef<T> = ArcSwapOption<RwLock<Node<T>>>;
type NodeRef<T> = Option<Arc<RwLock<Node<T>>>>;

#[derive(Default)]
struct Node<T> {
    data: T,
    next: NodeRef<T>,
    prev: NodeRef<T>,
}

impl<T> Node<T> {
    fn default_list_ref() -> ListRef<T> {
        ArcSwapOption::empty()
    }
}

struct DoublyLinkedList<T> {
    head: ListRef<T>,
    tail: ListRef<T>,
}

impl<T> DoublyLinkedList<T> {
    fn new() -> Self {
        Self {
            head: Node::default_list_ref(),
            tail: Node::default_list_ref(),
        }
    }

    /// I couldn't modify p.prev for head without RwLock
    fn prepend(&self, data: T) {
        let new_head = Node {
            data,
            prev: None,
            next: self.head.load_full(),
        };

        let prev_head = self.head.swap(Some(Arc::new(RwLock::new(new_head))));
        match prev_head {
            None => {}
            Some(p) => {
                p.write().unwrap().prev = self.head.load_full();
            }
        }

        if self.tail.load().is_none() {
            self.tail.store(self.head.load_full())
        }
    }

    fn append(&self, data: T) {
        let new_tail = Node {
            data,
            prev: self.tail.load_full(),
            next: None,
        };

        let prev_tail = self.tail.swap(Some(Arc::new(RwLock::new(new_tail))));
        match prev_tail {
            None => {}
            Some(p) => {
                p.write().unwrap().next = self.tail.load_full();
            }
        }

        if self.head.load().is_none() {
            self.head.store(self.tail.load_full())
        }
    }

    fn pop(&self) -> Option<T> {
        if self.head.load().is_none() {
            None
        } else {
            let new_head = self
                .head
                .load()
                .deref()
                .as_ref()
                .unwrap()
                .read()
                .unwrap()
                .next
                .clone();
            match &new_head {
                None => {}
                Some(p) => {
                    p.write().unwrap().prev = None;
                }
            }
            let old_head = self.head.swap(new_head).unwrap();

            Arc::into_inner(old_head).map(|lock| lock.into_inner().unwrap().data)
        }
    }

    fn into_iter(self) -> impl Iterator<Item = T>
    where
        T: Clone,
    {
        DoublyLinkedListIter {
            curr: self.head.load_full(),
        }
    }

    fn into_iter_rev(self) -> impl Iterator<Item = T>
    where
        T: Clone,
    {
        DoublyLinkedListIterReversed {
            curr: self.tail.load_full(),
        }
    }
}

impl<T> Clone for DoublyLinkedList<T> {
    fn clone(&self) -> Self {
        DoublyLinkedList {
            head: ArcSwapOption::from(self.head.load_full()),
            tail: ArcSwapOption::from(self.tail.load_full()),
        }
    }
}

struct DoublyLinkedListIter<T> {
    curr: Option<Arc<RwLock<Node<T>>>>,
}

impl<T: Clone> Iterator for DoublyLinkedListIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next = None;
        let data = match &self.curr {
            None => None,
            Some(node) => {
                let data = node.read().unwrap().data.clone();
                next = node.read().unwrap().next.clone();
                Some(data)
            }
        };

        self.curr = next;

        data
    }
}

struct DoublyLinkedListIterReversed<T> {
    curr: Option<Arc<RwLock<Node<T>>>>,
}

impl<T: Clone> Iterator for DoublyLinkedListIterReversed<T> {
    type Item = T;fn next(&mut self) -> Option<Self::Item> {
        let mut next = None;
        let data = match &self.curr {
            None => None,
            Some(node) => {
                let data = node.read().unwrap().data.clone();
                next = node.read().unwrap().prev.clone();
                Some(data)
            }
        };

        self.curr = next;

        data
    }
}

fn main() {
    let l = DoublyLinkedList::new();

    thread::scope(|s| {
        s.spawn(|| {
            //sleep(Duration::from_nanos(6));
            for i in 11..20 {
                l.prepend(i);
            }
        });

        s.spawn(|| {
            for i in 1..10 {
                //sleep(Duration::from_nanos(5));
                l.append(i);
            }
        });

        s.spawn(|| {
            for _ in 1..10 {
                sleep(Duration::from_nanos(1));
                for n in l.clone().into_iter_rev() {
                    print!("{}, ", n);
                }
                print!("\n\n");
            }
        });
    });
}
