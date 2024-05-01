use std::rc::Rc;

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    element: T,
    next: Link<T>,
}

pub struct List<T> {
    head: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn prepend(&self, element: T) -> List<T> {
        List {
            head: Some(Rc::new(Node {
                element,
                next: self.head.clone(),
            })),
        }
    }

    pub fn tail(&self) -> List<T> {
        /*
        if let Some(node) = &self.head  {
            let link = &node.next;
            if link.is_some() {
                return List { head: link.clone() };
            }
        }
        List { head: None }
        */

        List {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.element)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(node) = cur_link {
            if let Ok(mut node) = Rc::try_unwrap(node){
                cur_link = node.next.take();
            } else {
                break;
            }
        }
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.element
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics_1() {
        let list = List::new();
        assert_eq!(list.peek(), None);

        let list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.peek(), Some(&3));

        let list = list.tail();
        assert_eq!(list.peek(), Some(&2));

        let list = list.tail();
        assert_eq!(list.peek(), Some(&1));

        let list = list.tail();
        assert_eq!(list.peek(), None);

        let list = list.tail();
        assert_eq!(list.peek(), None);
    }

    #[test]
    fn basics_2() {
        let list1 = List::new();
        assert_eq!(list1.peek(), None);

        let list2 = list1.prepend(1).prepend(2).prepend(3);
        assert_eq!(list2.peek(), Some(&3));

        let list3 = list2.tail();
        assert_eq!(list3.peek(), Some(&2));

        let list4 = list3.tail();
        assert_eq!(list4.peek(), Some(&1));

        let list5 = list4.tail();
        assert_eq!(list5.peek(), None);

        let list6 = list5.tail();
        assert_eq!(list6.peek(), None);
    }

    #[test]
    fn iter_1() {
        let list = List::new().prepend(1).prepend(2).prepend(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn iter_2() {
        let list1 = List::new();
        let mut iter_list1 = list1.iter();

        let list2 = list1.prepend(1).prepend(2).prepend(3);
        let mut iter_list2 = list2.iter();

        let list3 = list2.tail();
        let mut iter_list3 = list3.iter();

        let list4 = list3.tail();
        let mut iter_list4 = list4.iter();

        let list5 = list4.tail();
        let mut iter_list5 = list5.iter();

        let list6 = list5.tail();
        let mut iter_list6 = list6.iter();

        assert_eq!(iter_list1.next(), None);
        assert_eq!(iter_list5.next(), None);
        assert_eq!(iter_list6.next(), None);

        assert_eq!(iter_list4.next(), Some(&1));
        assert_eq!(iter_list4.next(), None);

        assert_eq!(iter_list3.next(), Some(&2));
        assert_eq!(iter_list3.next(), Some(&1));
        assert_eq!(iter_list3.next(), None);

        assert_eq!(iter_list2.next(), Some(&3));
        assert_eq!(iter_list2.next(), Some(&2));
        assert_eq!(iter_list2.next(), Some(&1));
        assert_eq!(iter_list2.next(), None);
    }
}
