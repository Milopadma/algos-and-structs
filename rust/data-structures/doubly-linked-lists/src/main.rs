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

// now getting into the actual Doubly Linked List implementations

// struct for the DoublyLinkList
struct DoublyLinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    size: usize,
}

// implement for the struct
impl<T> DoublyLinkedList<T> {
    // for new doubly linked list
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            size: 0,
        }
    }

    // to check if the linkedlist is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    // to check the length of the linkedlist
    pub fn len(&self) -> usize {
        self.size
    }

    // push function here
    pub fn push_back(&mut self, item: T) {
        let node = Rc::new(RefCell::new(ListNode::new(item))); // this builds a new node
        if let Some(prev_tail) = self.tail.take() {
            //takes the value out of the option and puts it into a Some of the prev_tail
            prev_tail.borrow_mut().next = Some(Rc::clone(&node));
            node.borrow_mut().prev = Some(prev_tail);
            self.tail = Some(node);
            self.size += 1;
        } else {
            self.head = Some(Rc::clone(&node));
            self.tail = Some(node);
            self.size = 1;
        }
    }
}

// ref https://rtoch.com/posts/rust-doubly-linked-list/#:~:text=Doubly%20Linked%20List%20(or%20Linked,Stack%20%2C%20Queue%20%2C%20and%20Deque%20.