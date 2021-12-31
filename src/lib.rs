pub mod first;
pub mod second;
pub mod third;
pub mod fourth;
pub mod fifth_broken;
pub mod fifth;

#[cfg(test)]
mod first_tests {
    use std::rc::Rc;
    use crate::first::List;

    #[test]
    fn first_works() {
        let mut l = List::new();
        l.push(1);
        l.push(2);
        assert_eq!(l.pop(), Some(2));
        assert_eq!(l.pop(), Some(1));
        assert_eq!(l.pop(), None);
    }
}

#[cfg(test)]
mod second_tests {
    use std::rc::Rc;
    use crate::second::List;

    #[test]
    fn second_works() {
        let mut l = List::new();
        let s1 = Rc::new(String::from("abc"));
        let s2 = Rc::new(String::from("def"));
        l.push(s1.clone());
        l.push(s2.clone());
        assert_eq!(l.pop(), Some(s2));
        assert_eq!(l.pop(), Some(s1));
        assert_eq!(l.pop(), None);
    }

    #[test]
    fn peek() {
        let mut l = List::new();
        l.push(1);
        l.push(2);
        assert_eq!(l.peek(), Some(&2));
        assert_eq!(l.peek(), Some(&2));
    }

    #[test]
    fn peek_mut() {
        let mut l = List::new();
        let s1 = Rc::new(String::from("abc"));
        let s2 = Rc::new(String::from("def"));
        let s3 = Rc::new(String::from("ghi"));
        l.push(s1.clone());
        l.push(s2.clone());
        l.peek_mut().map(|it| { *it = s3.clone() });

        assert_eq!(l.peek_mut(), Some(&mut s3.clone()));
    }

    #[test]
    fn iter() {
        let mut l = List::new();
        l.push(1);
        l.push(2);
        let mut i = l.iter();
        assert_eq!(i.next(), Some(&2));
        assert_eq!(i.next(), Some(&1));
        assert_eq!(i.next(), None);
    }

    #[test]
    fn into_iter() {
        let mut l = List::new();
        l.push(1);
        l.push(2);
        let mut i = l.into_iter();
        assert_eq!(i.next(), Some(2));
        assert_eq!(i.next(), Some(1));
        assert_eq!(i.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut l = List::new();
        l.push(1);
        l.push(2);
        l.push(3);

        let mut i = l.iter_mut();
        let mut j = i.next().unwrap();
        *j = 5;

        let mut k = l.iter_mut();

        assert_eq!(k.next(), Some(&mut 5));
        assert_eq!(k.next(), Some(&mut 2));
        assert_eq!(k.next(), Some(&mut 1));
    }

}


#[cfg(test)]
mod third_tests {

    use std::rc::Rc;
    use crate::third::List;

    #[test]
    fn third_works() {
        let mut l = List::new();
        assert_eq!(l.head(), None);
        // assert_eq!(l.tail(), None);
        l.prepend(1);
        // assert_eq!(l.head(), Some(&1));

    }

    #[test]
    fn iter() {
        let mut l = List::new()
            .prepend(1)
            .prepend(2);
        let mut i = l.iter();
        assert_eq!(i.next(), Some(&2));
        assert_eq!(i.next(), Some(&1));
        assert_eq!(i.next(), None);
    }

}

#[cfg(test)]
mod fourth_tests {
    use std::rc::Rc;
    use crate::fourth::List;

    #[test]
    fn fourth_works() {
        let mut l = List::new();
        l.push_front(1);
        l.push_front(2);
        assert_eq!(l.pop_front(), Some(2));
        assert_eq!(l.pop_front(), Some(1));
        assert_eq!(l.pop_front(), None);

    }

     #[test]
    fn peek() {
        let mut l = List::new();
        l.push_front(1);
        l.push_front(2);
        assert_eq!(&*l.peek_front().unwrap(), &2);
        assert_eq!(&*l.peek_front().unwrap(), &2);
    }

    #[test]
    fn push_tail() {
         let mut l = List::new();
        l.push_front(1);
        l.push_back(2);
        assert_eq!(l.pop_front(), Some(1));
    }
}

#[cfg(test)]
mod fifth_broken_tests {
    use std::rc::Rc;
    use crate::fifth_broken::List;


/*
    #[test]
    fn broken() {
        let i = 0;
        let mut list = List::new(&i);

        // Check empty list behaves right
        // assert_eq!(list.pop(), None);

        // Populate list
        list.push();
        list.push();

    }
*/
}


#[cfg(test)]
mod fifth_tests {
    use std::rc::Rc;
    use crate::fifth::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);

        // Check the exhaustion case fixed the pointer right
        list.push(6);
        list.push(7);

        // Check normal removal
        assert_eq!(list.pop(), Some(6));
        assert_eq!(list.pop(), Some(7));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), None);
    }
}