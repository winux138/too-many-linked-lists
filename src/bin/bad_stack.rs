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
