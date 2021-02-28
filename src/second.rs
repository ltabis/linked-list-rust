// for the sake of being simple, we will skip genericity for this one
pub struct List<T> {
    head: Link<T>
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    value: T,
    next: Link<T>
}

impl<T> List<T> {
    fn new() -> Self {
        List { head: None }
    }

    fn push(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take()
        });

        self.head = Some(new_node);
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.value
        })
    }

    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.value
        })
    }

    fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.value
        })
    }
}

// we need to implemented drop manually because
// the regular drop would call recursivly all drops from
// all elements from the list.
// this could lead into a stack overflow.
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut curr_link = self.head.take();

        // we could also use while let Some(_) = self.pop() { }
        // but this is better.
        while let Some(mut boxed_node) = curr_link {
            curr_link = boxed_node.next.take();
        }
    }
}

// Indicates that we compile this part only when testing.
#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();
        let mut list2 = List::new();

        assert_eq!(list.pop(), None);

        list.push(42);
        list2.push("abcdef");
        assert_eq!(list2.pop(), Some("abcdef"));
        assert_eq!(list.pop(), Some(42));

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(1));
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        let mut list2 = List::new();
        assert_eq!(list2.peek(), None);

        list.push(5);
        list2.push("abcdef");

        assert_eq!(list.peek(), Some(&5));
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list2.peek_mut(), Some(&mut "abcdef"));
        assert_eq!(list2.pop(), Some("abcdef"));
    }
}