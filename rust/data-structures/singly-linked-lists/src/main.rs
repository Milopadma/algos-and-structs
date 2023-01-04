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
                *self = Link::Tail { data: x };
            }
            Self::Tail { data } => {
                // if the current node is a tail then create a new link
                *self = Link::Link {
                    data: *data,
                    next: Box::new(Link::Tail { data: x }),
                };
            }
            Self::Link { data, next } => {
                // if the current node is a link then push the new data onto the next node
                next.push(x);
            }
        }
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