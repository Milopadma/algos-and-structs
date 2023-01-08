// singly linked list
// 1 -> 2 -> 3 -> 4 -> 5 -> 6 -> 7 -> 8 -> 9 -> 10 -> null

// q: why is an enum used here?
// a: because we want to be able to represent the empty list, a tail, and a link
#[derive(Clone)]
enum Link<T> {
    Empty,
    Tail {
        data: T,
    },
    Link {
        data: T,
        next: Box<Link<T>>,
    },
}

// q: why is a trait bound used here?
// a: because we want to be able to copy the data into the list
impl<T> Link<T> where T: Copy {
    // impl<T> Link<T> where T: Copy means that the type T must implement the Copy trait
    pub fn push(&mut self, x: T) {
        // the &mut self is a reference to the current node in the list
        match self {
            // match the current node in the list to see if it is empty, a tail, or a link
            Self::Empty => {
                // if the current node is empty then create a new tail
                self.to_tail(x);
            }
            Self::Tail { .. } => {
                // if the current node is a tail then create a new link
                self.to_link(x);
            }
            Self::Link { next, .. } => {
                // if the current node is a link then push the new data onto the next node
                next.push(x);
            }
        }
    }

    //to turn a link into a Link::Tail variant
    fn to_tail(&mut self, it: T) {
        *self = match self {
            Self::Empty => Link::Tail { data: it },
            Self::Link { data: _, next: _ } => Self::Tail { data: it },

            _ => panic!("couldnt convert"),
        };
    }

    // to turn a link into a Link::Link variant
    fn to_link(&mut self, x: T) {
        *self = match self {
            // why is the match statement using *self? a: because we want to be able to replace the current node with a new node
            Self::Tail { data } =>
                Self::Link {
                    data: *data,
                    next: Box::new(Self::Tail { data: x }),
                },
            _ => panic!("couldnt convert"),
        };
    }
    // at this point, Push is implemented for the Link enum

    // now for the pop method
    pub fn pop(&mut self) -> Option<T> {
        // pop returns an Option type because the list may be empty
        match self {
            Self::Empty => None,
            Self::Tail { data } => {
                let data = *data; // copy the data into a new variable
                self.to_none(); // turn the tail into an empty node

                Some(data) // return the data
            }
            Self::Link { data, next } => {
                let mut n = Box::new(Self::Empty); // create a new empty node
                let data = *data; // copy the data into a new variable
                std::mem::swap(next, &mut n); // swap the next node with the empty node
                self.to_next(*n); // turn the current node into the next node

                Some(data) // return the data
            }
        }
    }

    // conversion functions
    fn to_none(&mut self) {
        *self = std::mem::replace(self, Link::Empty); // replace the current node with an empty node in memory
    }

    fn to_next(&mut self, nxt: Link<T>) {
        // q: why is the nxt parameter a Link<T> type? a: because we want to be able to replace the current node with the next node
        *self = nxt;
    }

    fn new() -> Self {
        Self::Empty
    }
}
// ref
// https://medium.com/swlh/implementing-a-linked-list-in-rust-c25e460c3676

// Cursor struct
#[derive(Clone)] // q: why is the Clone trait bound used here? a: because we want to be able to clone the cursor
struct Cursor<T> {
    curr: Link<T>,
}

impl<T> IntoIterator for Link<T> where T: Copy {
    // spawns a Cursor to iterate through the Link sequence
    type Item = T;
    type IntoIter = Cursor<T>;

    fn into_iter(self) -> Self::IntoIter {
        Cursor { curr: self }
    }
}

impl<T> Iterator for Cursor<T> where T: Copy {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let nxt = match self.curr {
            Link::Empty => None,
            Link::Tail { data } => {
                self.curr = Link::Empty;

                Some(data)
            }
            Link::Link { data, ref mut next } => {
                let mut n = Box::new(Link::Empty);
                std::mem::swap(next, &mut n);
                self.curr = *n;

                Some(data)
            }
        };
        nxt
    }
}
fn main() {
    // new linked list
    let mut list: Link<i32> = Link::new();

    list.push(1);
    list.push(2);
    list.push(3);

    println!("{:?}", list.pop().unwrap());
    println!("{:?}", list.pop().unwrap());
    println!("{:?}", list.pop().unwrap());

    // create a new node
    // let mut head = Node {
    //     data: 1,
    //     next: None,
    // };

    // // create a new node
    // let node2 = Node {
    //     data: 2,
    //     next: None,
    // };

    // // create a new node
    // let node3 = Node {
    //     data: 3,
    //     next: None,
    // };

    // // link the nodes
    // head.next = Some(Box::new(node2));
    // head.next.as_mut().unwrap().next = Some(Box::new(node3));

    // // print the linked list
    // let mut current = &head;
    // while let Some(node) = current.next.as_ref() {
    //     println!("{}", current.data);
    //     current = node;
    // }
}