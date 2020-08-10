use std::sync::{Arc, Mutex, MutexGuard};
use std::mem;

pub struct ConcurrentLinkedList<T> {
    node: Arc<Mutex<Option<Node<T>>>>
}

struct Node<T> {
    next: Arc<Mutex<Option<Node<T>>>>,
    value: Option<T>,
}

impl<T> Node<T> {
    fn value_as_ref(&self) -> Option<&T> {
        self.value.as_ref()
    }

    fn get_next(&self) -> Arc<Mutex<Option<Node<T>>>> {
        Arc::clone(&self.next)
    }

}

fn arc_mut_new<T>(value: T) -> Arc<Mutex<T>> {
    Arc::new(Mutex::new(value))
}

impl<T> ConcurrentLinkedList<T> {
    pub fn new() -> Self {
        Self { node: arc_mut_new((None)) }
    }

    pub fn new_from(value: T) -> Self {
        let node = Node { next: arc_mut_new(None), value: Some(value) };
        Self { node: arc_mut_new(Some(node)) }
    }

    pub fn contains(&self, like: &T) -> bool
        where T: Eq,
    {
        // let mut previous = arc_mut_new(None);
        // let mut next = Arc::clone(&self.node);
        let mut next: MutexGuard<Option<Node<T>>> = self.node.lock().unwrap();
        let mut previous: MutexGuard<Option<Node<T>>>;
        let mut aux: MutexGuard<Option<Node<T>>>;
        let mut arc_next;
        loop {
            if next.is_none() {
                break false;
            }
            if next.as_ref().unwrap().value_as_ref() == Some(like) {
                break true;
            }
            // Not found yet, checking next elements
            arc_next = next.as_ref().unwrap().get_next();
            previous = next;

            // SAFETY:
            // Safe since we moved the next node into arc_next
            // and the current node into previous,
            // so now we just take the lock for arc_next and move it into variable `next`
            // so that it holds the MutexGuard
            next = unsafe { mem::transmute(arc_next.lock().unwrap()) };
        }

    }
}
