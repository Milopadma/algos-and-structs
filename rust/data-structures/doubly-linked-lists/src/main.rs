// doubly linked list
// same as singly linked list but with a prev pointer to point to the node before it

use std::{ cell::RefCell, rc::Rc };

// this struct acts as a Node in the list
// its a struct because it has multiple fields of related properties
struct ListNode<T> {
    item: T,
    next: Link<T>,
    prev: Link<T>,
}

// this is a type alias for the Option<Rc<RefCell<ListNode<T>>>>
type Link<T> = Option<Rc<RefCell<ListNode<T>>>>;

// implement the struct
impl<T> ListNode<T> {
    // new function to create a new empty ListNode
    fn new(item: T) -> Self {
        Self {
            item,
            next: None,
            prev: None, // this was not in the singly linked list
        }
    }
}

// ref https://rtoch.com/posts/rust-doubly-linked-list/#:~:text=Doubly%20Linked%20List%20(or%20Linked,Stack%20%2C%20Queue%20%2C%20and%20Deque%20.