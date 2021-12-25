use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn prepend(&self, elem: T) -> List<T> {
        let node = Node<T> { elem, self.head.clone() };
        List {
           head: Some(Rc::new(node))
        }
    }

    pub fn tail(&self) -> List<T> {
        List {
            head: self.head.as_ref()
                .map(|node| node.next.clone() )
        }
    }
}

