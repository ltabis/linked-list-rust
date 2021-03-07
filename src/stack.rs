use std::rc::Rc;

pub struct Stack<T> {
    head: Link<T>
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    value: T,
    next: Link<T>
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { head: None }
    }

    pub fn append(&self, value: T) -> Self {
        Stack {
            head: Some(Rc::new(Node {
                value,
                next: self.head.clone()
            }))
        }
    }

    pub fn tail(&self) -> Self {
        Stack { head: self.head.as_ref().and_then(|node| node.next.clone()) }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }
}

#[cfg(test)]
mod test {
    use super::Stack;

    #[test]
    fn basics() {
        let stack = Stack::new();
        assert_eq!(stack.head(), None);

        let stack = stack
            .append(1)
            .append(2)
            .append(3);
        assert_eq!(stack.head(), Some(&3));

        let stack = stack.tail();
        assert_eq!(stack.head(), Some(&2));

        let stack = stack.tail();
        assert_eq!(stack.head(), Some(&1));

        let stack = stack.tail();
        assert_eq!(stack.head(), None);

        let stack = stack.tail();
        assert_eq!(stack.head(), None);
    }
}