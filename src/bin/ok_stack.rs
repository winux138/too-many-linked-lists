use std::mem;

#[derive(Debug)]
struct List {
    head: Link,
}

#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Debug)]
struct Node {
    value: i32,
    next: Link,
}

impl List {
    const fn new() -> Self {
        Self { head: Link::Empty }
    }

    fn push(&mut self, v: i32) {
        self.head = Link::More(Box::new(Node {
            value: v,
            next: mem::replace(&mut self.head, Link::Empty),
        }));
    }

    fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.value)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
            // boxed_node goes out of scope and gets dropped here;
            // but its Node's `next` field has been set to Link::Empty
            // so no unbounded recursion occurs.
        }
    }
}

fn main() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    list.push(4);
    println!("poping: {:?}", list.pop());
    println!("poping: {:?}", list.pop());
    println!("poping: {:?}", list.pop());
    println!("poping: {:?}", list.pop());
    println!("poping: {:?}", list.pop());
    println!("list: {list:?}");
}
