pub mod first;
pub mod second;
pub mod third;


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
