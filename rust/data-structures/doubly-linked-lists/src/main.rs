// doubly linked list
// same as singly linked list but with a prev pointer to point to the node before it

use std::{ cell::RefCell, rc::Rc };

struct ListNode<T> {
    item: T,
    next: Link<T>,
    prev: Link<T>,
}

type Link<T> = Option<Rc<RefCell<ListNode<T>>>>;

// ref https://rtoch.com/posts/rust-doubly-linked-list/#:~:text=Doubly%20Linked%20List%20(or%20Linked,Stack%20%2C%20Queue%20%2C%20and%20Deque%20.