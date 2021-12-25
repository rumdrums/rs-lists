type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub struct List<T> {
    head: Link<T>,
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take()
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take()
            .map(|node| {
                self.head = node.next;
                node.elem
            })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }


    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }

    pub fn iter(&self) -> Iter< T> {
        // Iter { next: self.head.as_ref().map(|node| node.as_ref() ) }
        Iter { next: self.head.as_deref() }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut { next: self.head.as_deref_mut() }
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

}

impl<'a, T> Iterator for Iter<'a, T> {

    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {

        self.next.map(|node: &Node<T>| -> &T {



            // option 1
            // let y: Option<&Node<T>> = node.next.as_ref()
            //     .map(|node: &Box<Node<T>>| -> &Node<T> {
            //         node.as_ref()
            //         // equivalent to (*node).as_ref()
            //     });

            // self.next = y;
            /*
            node is &Node
            node.next is Option<Box<Node<T>>>
            .as_ref() is Option<&Box<Node<T>>>
            .map is &Box<Node<T>>
            .as_ref is &Node<T>
            self.next = Option<&Node<T>>
            */
            // end option 1

            // alternate way to express option 1:
            self.next = match node.next.as_ref() {
                Some(n) => Some(n.as_ref()),
                None => None
            };
            /*
            node.next is Option<Box<Node<T>>>
            .as_ref() is Option<&Box<Node<T>>>
            return Option<&Node<T>>
            or None
            */
            ///// end alternate option 1

            // option 2
            // let y: Option<&Node<T>> = node.next.as_deref();
            // self.next = y;
            /*
            node is &Node
            node.next is Option<Box<Node<T>>>
            .as_deref() is Option<&Node<T>>
            */
            // end option 2

            &node.elem

        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // access fields of a tuple struct numerically
        self.0.pop()
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();

        while let Some(mut boxed_node) = cur_link {
            cur_link = self.head.take();
        }
    }
}

