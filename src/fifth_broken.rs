
pub struct List<'a>(&'a i32);

impl<'a> List<'a> {

    pub fn new(elem: &'a i32) -> Self {
        List(elem)
    }

    /// ***** Just having the same lifetime here as `tail`
    /// results in 'multiple borrow' problem:
    pub fn push(&'a mut self) {
        // let new_tail = Box::new(Node {
        //     elem: elem,
        //     next: None,
        // });

        // let new_tail = match self.tail.take() {
        //     Some(old_tail) => {
        //         old_tail.next = Some(new_tail);
        //         old_tail.next.as_deref_mut()
        //     }
        //     None => {
        //         self.head = Some(new_tail);
        //         self.head.as_deref_mut()
        //     }
        // };

        // self.tail = new_tail;

    }

    // pub fn pop(&'a mut self) -> Option<T> {
    //     self.head.take().map(|head| {
    //         let head = *head;
    //         self.head = head.next;

    //         if self.head.is_none() {
    //             self.tail = None;
    //         }

    //         head.elem
    //     })
    // }

}