#[derive(Debug)]
struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Link<T>,
}

impl<T> List<T> {
    const fn new() -> Self {
        Self { head: Option::None }
    }

    fn push(&mut self, v: T) {
        self.head = Option::Some(Box::new(Node {
            value: v,
            next: self.head.take(),
        }));
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.value
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Option::Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

fn main() {
    let mut list = List::new();
    list.push(1u16);
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
