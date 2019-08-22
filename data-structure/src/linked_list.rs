use std::rc::Rc;
use std::cell::RefCell;
use std::fmt::Debug;

#[derive(Debug, Eq, PartialEq)]
struct Node<T: Debug> {
    data: T,
    next: Option<Rc<RefCell<Node<T>>>>
}

//impl<T> Node<T> 
//where T: Debug
//{
//    pub fn get_next(&self) -> Option<Self> {
//        match self.next {
//            Some(node) => {
//                *node.
//                Some()
//            },
//            None => None,
//        }
//    }
//}

#[derive(Debug)]
pub struct LinkedList<T: Debug> {
    length: usize,
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>
}

impl<T> LinkedList<T>
where T: Debug + Copy
{
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            length: 0
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn get_head(&self) -> Option<T> {
        match &self.head {
            Some(head) => {
                Some(head.borrow().data)
            },
            None => None
        }
    }

    pub fn get_tail(&self) -> Option<T> {
        match &self.tail {
            Some(tail) => {
                Some(tail.borrow().data)
            },
            None => None
        }
    }

    pub fn append(&mut self, data: T) {
        let node = Rc::new(RefCell::new(Node { data, next: None }));

        if let Some(current_tail) = &mut self.tail {
            current_tail.borrow_mut().next = Some(node.clone());
            self.tail = Some(node.clone());
        } else {
            self.head = Some(node.clone());
            self.tail = Some(node.clone());
        }

        self.length += 1;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_len() {
        let list = LinkedList::<usize>::new();
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_len2() {
        let mut list = LinkedList::new();
        list.append(1);
        list.append(1);
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_get_head_none() {
        let list = LinkedList::<usize>::new();
        assert_eq!(list.get_head(), None);
    }

    #[test]
    fn test_get_head() {
        let mut list = LinkedList::new();
        list.append(1);
        assert_eq!(list.get_head(), Some(1));
    }

    #[test]
    fn test_get_tail_none() {
        let list = LinkedList::<usize>::new();
        assert_eq!(list.get_tail(), None);
    }

    #[test]
    fn test_get_tail() {
        let mut list = LinkedList::new();
        list.append(1);
        list.append(2);
        assert_eq!(list.get_tail(), Some(2));
    }

    #[test]
    fn test_append() {
        let mut list = LinkedList::new();
        list.append(1);
        assert_eq!((*list.head.unwrap()).borrow().data, 1);
    }
}
