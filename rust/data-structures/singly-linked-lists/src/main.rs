// singly linked list
// 1 -> 2 -> 3 -> 4 -> 5 -> 6 -> 7 -> 8 -> 9 -> 10 -> null

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

impl<T> Link<T> where T: Copy {
    fn push(&mut self, x: T) {
        match self {
            Link::Empty => {
                *self = Link::Tail { data: x };
            }
            Link::Tail { data } => {
                *self = Link::Link {
                    data: *data,
                    next: Box::new(Link::Tail { data: x }),
                };
            }
            Link::Link { data, next } => {
                next.push(x);
            }
        }
    }
}
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