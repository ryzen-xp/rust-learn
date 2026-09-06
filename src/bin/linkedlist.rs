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
        match self.head.as_mut() {
            Some(x) => {
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

            None => (),
        }
    }

    fn length(&self) -> usize {
        // match self.head.as_mut() {
        //     Some(mut x) => {
        //         let mut counter = 1;

        //         while let Some(y) = x.next {
        //             counter += 1;

        //             if y.next.is_none() {
        //                 break;
        //             }

        //             x = y;
        //         }

        //         counter
        //     }

        //     None => 0,
        // }
        let mut counter = 0;

        match self.head.as_mut() {
            None => 0,

            Some(x) => {
                loop {
                    if x.next.is_none() {
                        break;
                    }
                }

                counter
            }
        }
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
    }
}
