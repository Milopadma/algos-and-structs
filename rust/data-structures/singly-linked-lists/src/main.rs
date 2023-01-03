// singly linked list
// 1 -> 2 -> 3 -> 4 -> 5 -> 6 -> 7 -> 8 -> 9 -> 10 -> null

struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

fn main() {
    // create a new node
    let mut head = Node {
        data: 1,
        next: None,
    };

    // create a new node
    let node2 = Node {
        data: 2,
        next: None,
    };

    // create a new node
    let node3 = Node {
        data: 3,
        next: None,
    };

    // link the nodes
    head.next = Some(Box::new(node2));
    head.next.as_mut().unwrap().next = Some(Box::new(node3));

    // print the linked list
    let mut current = &head;
    while let Some(node) = current.next.as_ref() {
        println!("{}", current.data);
        current = node;
    }
}