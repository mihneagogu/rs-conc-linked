use std::sync::{Arc, Mutex};
use std::thread;

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
        }).join();
    }

    #[test]
    fn find_one() {
        let list = ConcurrentLinkedList::new_from(12);
        let list = Arc::new(list);
        let list_arc = Arc::clone(&list);
        thread::spawn(move || {
            let (head, tail) = list_arc.find(&12);
            assert!(head.is_none());
            assert!(tail.is_some());
        }).join();
    }

    #[test]
    fn find_two() {
        let list = ConcurrentLinkedList::new_from_two(1, 2);
        let list = Arc::new(list);
        let list_arc = Arc::clone(&list);
        let list_arc_2 = Arc::clone(&list);
        thread::spawn(move || {
            let (head, tail) = list_arc_2.find(&2);
            assert!(head.is_some());
            assert!(tail.is_some());
        }).join();
    }

    #[test]
    fn contains_two() {
        let list = ConcurrentLinkedList::new_from_two(1, 2);
        let list = Arc::new(list);
        let list_arc = Arc::clone(&list);
        thread::spawn(move || {
            assert!(list_arc.contains(&1));
            assert!(list_arc.contains(&2));
        }).join();
    }

    #[test]
    fn contains_one_push() {
        let list = ConcurrentLinkedList::new();
        let list = Arc::new(list);
        let list_arc = Arc::clone(&list);
        thread::spawn(move || {
            list.push(12);
        })
        .join();

        assert!(list_arc.contains(&12));
    }

    #[test]
    fn test_push() {
        let list = ConcurrentLinkedList::new();
        let list = Arc::new(list);
        let c_list = Arc::clone(&list);
        let arc_list = Arc::clone(&list);
        thread::spawn(move || {
            (0..100).for_each(|n| {
                arc_list.push(n);
            });
        }).join();

        thread::spawn(move || {
            (100..=200).for_each(|n| {
                c_list.push(n);
            });
        }).join();

        (0..=200).for_each(|n| {
            assert!(list.contains(&n));
        });
    }

    #[test]
    fn test_push_chaotic() {
        let list = ConcurrentLinkedList::new();
        let list = Arc::new(list);

        let mut threads = vec![];
        let nthreads = 100;
        (0..=nthreads).for_each(|n| {
            let arc_list = Arc::clone(&list);
            threads.push(thread::spawn(move || {
               arc_list.push(n); 
            }));
        });

        for thread in threads {
            let _ = thread.join();
        }

        (0..=nthreads).for_each(|n| { assert!(list.contains(&n)); });
        assert!(!list.contains(& (nthreads + 1) ));


    }
}
