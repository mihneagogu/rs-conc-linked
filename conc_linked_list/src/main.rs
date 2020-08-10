use std::thread;
use std::sync::{Arc, Mutex};

mod concurrent_linked_list;
use concurrent_linked_list::*;
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::concurrent_linked_list as lists;
    use lists::*;

    #[test]
    fn contains_one() {
        let list = ConcurrentLinkedList::new_from(12);
        let list = Arc::new(list);
        let list_arc = Arc::clone(&list);
        thread::spawn(move || {
            assert!(list_arc.contains(&12));
        });
    }
}
