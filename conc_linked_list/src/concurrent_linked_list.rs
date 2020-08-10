use std::sync::{Arc, Mutex};

pub struct ConcurrentLinkedList<T> {
    node: Arc<Mutex<Option<Node<T>>>>
}

struct Node<T> {
    next: Arc<Mutex<Option<Node<T>>>>,
    value: Option<T>,
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
        let mut previous = arc_mut_new(None);
        let mut next = Arc::clone(&self.node);
        loop {
          let arc_node = next.lock().unwrap();
          let next_node = (*arc_node).as_ref().unwrap();
          if next_node.value.as_ref().unwrap() == like {
               return true;
          }
          let next = Arc::clone(&next_node.next);
          previous = Arc::clone(&next);
        }

        false

    }

}
