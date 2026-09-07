// Topic: linkedlist
// Run with:  cargo run --bin linkedlist

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug, PartialEq, Eq)]
pub struct Node<T> {
    pub value: T,
    pub next: Link<T>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct LinkedList<T> {
    head: Link<T>,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn push_front(&mut self, value: T) {
        let node = Box::new(Node {
            value,
            next: self.head.take(),
        });

        self.head = Some(node);
    }

    fn push_back(&mut self, value: T) {
        let node = Box::new(Node { value, next: None });

        match self.head.as_mut() {
            None => {
                self.head = Some(node);
            }

            Some(mut x) => {
                while let Some(ref mut next) = x.next {
                    x = next;
                }

                x.next = Some(node);
            }
        }
    }

    fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|x| {
            self.head = x.next;
            x.value
        })
    }

    fn pop_back(&mut self) {
        if let Some(x) = self.head.as_mut() {
            if x.next.is_none() {
                self.head = None;
                return;
            }

            let mut current: &mut Box<Node<T>> = x;

            loop {
                if current.next.as_ref().unwrap().next.is_none() {
                    current.next = None;
                    break;
                }

                current = current.next.as_mut().unwrap();
            }
        }
    }

    /*
      LL =  head--> Some(Node)---->Some(Node)---->Some(Node)---->None
         head
           |
      x =  2-->5-->34-->4-->None


    */

    fn remove(&mut self, x: T)
    where
        T: PartialEq,
    {
        let mut current = &mut self.head;

        while current.is_some() {
            if current.as_ref().unwrap().value == x {
                let next = current.as_mut().unwrap().next.take();
                *current = next;
            } else {
                current = &mut current.as_mut().unwrap().next;
            }
        }
    }

    fn length(&self) -> usize {
        let mut counter = 0;

        let mut current = self.head.as_ref();

        while let Some(x) = current {
            counter += 1;

            current = x.next.as_ref();
        }

        counter
    }
}

fn main() {
    println!("Genric Linked List impl");
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty_list_length() {
        let ll: LinkedList<i32> = LinkedList::new();

        assert_eq!(ll.length(), 0);
    }

    #[test]
    fn test_one_element_length() {
        let mut ll = LinkedList::new();

        ll.push_back(10);

        assert_eq!(ll.length(), 1);
    }

    #[test]
    fn test_multiple_elements_length() {
        let mut ll = LinkedList::new();

        ll.push_back(1);
        ll.push_back(2);
        ll.push_back(3);
        ll.push_back(4);

        assert_eq!(ll.length(), 4);
    }

    #[test]
    fn test_push_front_length() {
        let mut ll = LinkedList::new();

        ll.push_front(1);
        ll.push_front(2);
        ll.push_front(3);

        assert_eq!(ll.length(), 3);
        ll.push_back(12);
        assert_eq!(ll.length(), 4);
        ll.remove(2);
        assert_eq!(ll.length(), 3);
    }
}
