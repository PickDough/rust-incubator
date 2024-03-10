use std::sync::{Arc, RwLock};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

use arc_swap::access::Access;
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
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
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
        let new_head = ArcSwapOption::from_pointee(RwLock::new(Node {
            data,
            prev: None,
            next: None,
        }));
        
        if self.tail.load().is_none() {
            self.tail.rcu(|rcu| {
                if rcu.is_none() {
                    new_head.load_full()
                } else {
                    rcu.clone()
                }
            });
        }

        self.head.rcu(|rcu| {
            match new_head.load().clone() {
                None => {}
                Some(new) => {
                    new.write().unwrap().next = rcu.clone();
                }
            }

            match rcu.clone() {
                None => {}
                Some(p) => {
                    p.write().unwrap().prev = new_head.load_full();
                }
            }

            new_head.load_full()
        });
    }

    fn append(&self, data: T) {
        let new_tail = ArcSwapOption::from_pointee(RwLock::new(Node {
            data,
            prev: None,
            next: None,
        }));

        if self.head.load().is_none() {
            self.head.rcu(|rcu| {
                if rcu.is_none() {
                    new_tail.load_full()
                } else {
                    rcu.clone()
                }
            });
        }

        self.tail.rcu(|rcu| {
            match new_tail.load().clone() {
                None => {}
                Some(new) => {
                    new.write().unwrap().prev = rcu.clone();
                }
            }

            match rcu.clone() {
                None => {}
                Some(p) => {
                    p.write().unwrap().next = new_tail.load_full();
                }
            }

            new_tail.load_full()
        });
    }

    fn pop(&self) -> Option<T> {
        if self.head.load().is_none() || self.tail.load().is_none() {
            None
        } else {
            let old_head = self.head.rcu(|rcu| {
                let next = rcu.clone().unwrap().read().unwrap().next.clone();
                match next.clone() {
                    None => {}
                    Some(p) => {
                        p.write().unwrap().prev = None;
                    }
                }

                next
            });

            Arc::into_inner(old_head.unwrap()).map(|lock| lock.into_inner().unwrap().data)
        }
    }

    fn into_iter(self) -> impl Iterator<Item=T>
        where
            T: Clone,
    {
        DoublyLinkedListIter {
            curr: self.head.load_full(),
        }
    }

    fn into_iter_rev(self) -> impl Iterator<Item=T>
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

fn main() {
    let l = DoublyLinkedList::new();

    thread::scope(|s| {
        s.spawn(|| {
            for i in 8..15 {
                l.append(i);
            }
        });

        s.spawn(|| {
            for i in (1..8).rev() {
                l.prepend(i);
            }
        });

        s.spawn(|| {
            sleep(Duration::from_nanos(500));
            for _ in 1..4 {
                println!("{:?}", l.pop());
            }
        });

        s.spawn(|| {
            sleep(Duration::from_millis(1000));
            for n in l.clone().into_iter() {
                print!("{}, ", n);
            }
            print!("\n\n\n");
            for n in l.clone().into_iter_rev() {
                print!("{}, ", n);
            }
        });
    });
}
