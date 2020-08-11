use std::mem;
use std::sync::{Arc, Mutex, MutexGuard};

/// Basically a concurrent stack with no pop method
/// and with linear contains method, which is using locks
/// However, `contains` does not guarantee that the item is not inside,
/// since another thread may have added the element at the beginning
#[derive(Debug)]
pub struct ConcurrentLinkedList<T> {
    node: Arc<Mutex<Option<Node<T>>>>,
}

#[doc = "Uses the type $t to construct type MutexGuard<Option<Node<T>>> 
since the standard library doesn't let me do this inside an impl block"]
macro_rules! node_guard {
    ($t:ty) => {
        MutexGuard<Option<Node<$t>>>
    }
}

#[derive(Debug)]
pub struct Node<T> {
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

    fn into_items(self) -> (Arc<Mutex<Option<Node<T>>>>, Option<T>) {
        (self.next, self.value)
    }
}

fn arc_mut_new<T>(value: T) -> Arc<Mutex<T>> {
    Arc::new(Mutex::new(value))
}

impl<T> ConcurrentLinkedList<T> {
    /// Constructs an empty list
    pub fn new() -> Self {
        Self {
            node: arc_mut_new(None),
        }
    }

    /// Constructs a list with a single element
    pub fn new_from(value: T) -> Self {
        let node = Node {
            next: arc_mut_new(None),
            value: Some(value),
        };
        Self {
            node: arc_mut_new(Some(node)),
        }
    }

    /// Constructor from two values, just for testing purposes
    pub fn new_from_two(head: T, tail: T) -> Self {
        let tail = Node {
            next: arc_mut_new(None),
            value: Some(tail),
        };
        let head = Node {
            next: arc_mut_new(Some(tail)),
            value: Some(head),
        };
        Self {
            node: arc_mut_new(Some(head)),
        }
    }


    /// Removes which items happens to be at the top of the list
    /// If used across threads it is guaranteed to retrieve the items in the order which they were inserted in
    /// HOWEVER! If the order of insertion is controlled by the scheduler, there is no guarantee that the order
    /// the scheduler chose is the order the user chose
    pub fn remove_one(&self) -> Option<T> {
        let mut head = self.node.lock().unwrap();
        if head.is_none() {
            return None;
        }
        // Take the head items, so that when trying to unwrap the arc we are guaranteed to have on 1 reference
        // to Arc, so unwrapping works
        let (arc_next, item) = head.take().unwrap().into_items();
        let new_node = Arc::try_unwrap(arc_next);

        // Shouldn't need this since we have already locked first node, so there is nobody else
        if new_node.is_err() {
            return None;
        }

        let new_node = new_node.unwrap_or(Mutex::new(None));
        // new_node.lock().unwrap();

        // Take the value under mutex, since we know we're the only owner
        let new_node = new_node.into_inner().unwrap();
        if new_node.is_none() {
            // Only 1 element in list
            // So the current head is just `None` since
            head.replace(Node {
                next: Arc::new(Mutex::new(None)),
                value: None,
            });
            return item;
        }
        head.replace(new_node.unwrap());
        item
    }

    /// Adds the item to the list, but it does not guarantee order if used across many threads,
    /// since the scheduler is free to choose which value gets entered first, the only thing we guarantee
    /// is that the all values pushed do end up in the List after all, not its order
    #[allow(dead_code)]
    pub fn push(&self, item: T)
    where T: std::fmt::Debug
    {
        // TODO: Continue function
        let mut head = self.node.lock().unwrap();
        if head.is_none() {
            // Empty list
            // println!("pushed: {:?}", &item);
            let first_node = Node {
                next: arc_mut_new(None),
                value: Some(item),
            };
            head.replace(first_node);
            return;
        }
        let previous_head = head.take();
        // println!("pushed: {:?}", &item);
        let new_head = Node {
            next: arc_mut_new(previous_head),
            value: Some(item),
        };
        head.replace(new_head);
    }

    /// Finds the position of the node that contains 'like', and returns the node
    /// and the node before it, if any
    #[allow(dead_code)]
    pub fn find(&self, like: &T) -> (Option<node_guard![T]>, Option<node_guard![T]>)
        where
            T: Eq,
    {
        let mut next: MutexGuard<Option<Node<T>>> = self.node.lock().unwrap();
        let mut is_first = true;
        let mut previous: Option<MutexGuard<Option<Node<T>>>> = None;
        let mut arc_next;
        loop {
            if next.is_none() {
                // List is actually empty
                // but the list still contains an Arc<Mutex<Option<Node<T>>>>
                // with None inside
                break (None, Some(next));
            }
            if next.as_ref().unwrap().value_as_ref() == Some(like) {
                break (previous, Some(next));
            }
            // Not found yet, checking next elements
            arc_next = next.as_ref().unwrap().get_next();
            let locked_next = arc_next.lock().unwrap();
            if locked_next.is_none() {
                if is_first {
                    break (None, Some(next));
                }
                break (previous, Some(next));
            }
            drop(locked_next);
            previous = Some(next);

            // SAFETY:
            // Safe since we moved the next node into arc_next
            // and the current node into previous,
            // so now we just take the lock for arc_next and move it into variable `next`
            // so that it holds the MutexGuard
            next = unsafe { mem::transmute(arc_next.lock().unwrap()) };
            is_first = false;
        }
    }

    /// Checks whether the 'like' is inside the list,
    /// but not guaranteed to give a consisten snapshot since
    /// someone can add the item at the end while the thread is searching
    /// (If this were a sorted linked list implementation this would be guaranteed)
    /// But it is not here
    #[allow(dead_code)]
    pub fn contains(&self, like: &T) -> bool
        where
            T: Eq,
    {
        // TODO: Change function using find()
        let mut next: MutexGuard<Option<Node<T>>> = self.node.lock().unwrap();
        let mut previous: MutexGuard<Option<Node<T>>>;
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
