// data structurte singly link list
class Node {
  next: Node | null = null;
  constructor(public data: string) {}
}

class SinglyLinkedList {
  constructor(public head: Node | null = null) {
    this.head = head;
    this.tail = tail;
  }
  add(data: string): void {
    const node = new Node(data);
    if (!this.head) {
      this.head = node;
      return;
    }
    let tail = this.head;
    while (tail.next) {
      tail = tail.next;
    }
    tail.next = node;
  }
}

//bad way to do it
var first = new Node("Hi");
first.next = new Node("there");
first.next.next = new Node("how");
first.next.next.next = new Node("are");
first.next.next.next.next = new Node("you");

//better way to do it
var list = new SinglyLinkedList(first);
list.add("doing");

export const Section12 = () => {};
