// singly linked list
// 1 -> 2 -> 3 -> 4 -> 5 -> 6 -> 7 -> 8 -> 9 -> 10 -> null

// q: why is an enum used here?
// a: because we want to be able to represent the empty list, a tail, and a link
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
    fn push(&mut self, x: T) {
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
        match self {
            Self::Empty => {
                *self = Link::Tail { data: it };
            }
            Self::Link { data: _, next: _ } => {
                *self = Link::Tail { data: it };
            }
            _ => {
                panic!("couldnt convert");
            }
        }
    }

    // to turn a link into a Link::Link variant
    fn to_link(&mut self, x: T) {
        match self {
            Self::Tail { data: _ } => {
                *self = Link::Link {
                    data: x,
                    next: Box::new(Link::Empty),
                };
            }
            _ => {
                panic!("couldnt convert");
            }
        }
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
            Self::Link { data, next } => {/*? */}
        }
    }

    // conversion functions
    fn to_none(&mut self) {
        *self = std::mem::replace(self, Link::Empty); // replace the current node with an empty node in memory
    }
}
// ref
// https://medium.com/swlh/implementing-a-linked-list-in-rust-c25e460c3676

fn main() {
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