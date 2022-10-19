// data structurte singly link list
class Node {
  next: Node | null = null;
  constructor(public data: string) {}
}

class SinglyLinkedList {
  constructor(
    public head: Node | null = null,
    public tail: Node | null,
    public length: number = 0
  ) {
    this.head = head;
    this.tail = tail;
    this.length = length;
  }
  add(data: string): SinglyLinkedList {
    const node = new Node(data);
    // if there is no head, set the head and
    // tail to be the newly created node
    if (!this.head) {
      this.head = node;
      this.tail = this.head;
      // otherwise set the next property on the tail
      // to be the new node and set the tail property on
      // the list to be the newly created node
    } else {
      this.tail!.next = node;
      this.tail = node;
    }

    this.length++;
    return this;
  }
}

//bad way to do it
var first = new Node("Hi");
first.next = new Node("there");
first.next.next = new Node("how");
first.next.next.next = new Node("are");
first.next.next.next.next = new Node("you");

//better way to do it
var list = new SinglyLinkedList(new Node("Hi"), new Node("there"));
list.add("doing");

export const Section12 = () => {};
