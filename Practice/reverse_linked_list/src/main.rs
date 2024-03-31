#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node { value, next: None }
    }
}

fn reverse_link_list<T>(mut head: Option<Box<Node<T>>>) -> Option<Box<Node<T>>> {
    let mut prev = None;
    while let Some(mut node) = head.take() {
        let next = node.next.take();
        node.next = prev.take();
        prev = Some(node);
        head = next;
    }
    prev
}
fn main() {
    let mut head = Some(Box::new(Node::new(1)));
    head.as_mut().unwrap().next = Some(Box::new(Node::new(2)));
    head.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(Node::new(3)));
    head.as_mut()
        .unwrap()
        .next
        .as_mut()
        .unwrap()
        .next
        .as_mut()
        .unwrap()
        .next = Some(Box::new(Node::new(4)));
    println!("{:?}", head);
    let res = reverse_link_list(head);
    println!("{:?}", res);
}
