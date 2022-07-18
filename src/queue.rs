use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct QNode {
    val: u64,
    next: Option<Rc<RefCell<QNode>>>,
}

pub struct Queue {
    head: Option<Rc<RefCell<QNode>>>,
    tail: Option<Rc<RefCell<QNode>>>,
}

impl Queue {
    pub fn push(&mut self, v: u64) {
        match &self.tail {
            None => {
                let node = Rc::new(RefCell::new(QNode{val: v, next: None}));
                self.tail = Some(Rc::clone(&node));
                self.head = Some(Rc::clone(&node));
            }
            Some(t) => {
                let node = Rc::new(RefCell::new(QNode{val: v, next: None}));
                {
                    let mut reference = t.borrow_mut();
                    reference.next = Some(Rc::clone(&node));
                }
                self.tail = Some(Rc::clone(&node));
            }
        }
    }

    pub fn pop(&mut self) -> Option<u64> {
        match &self.head {
            None => None,
            Some(h) => {
                let v: u64;
                let mut a = None;
                {
                    let borrow = (*h).borrow();
                    v = borrow.val;
                    match &borrow.next {
                        None => {
                            self.tail = None;
                        }
                        Some(n) => {
                            a = Some(Rc::clone(n));
                        }
                    }
                }
                self.head = a;
                Some(v)
            }
        }
    }

    pub fn new() -> Self {
        Queue{
            head: None,
            tail: None,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        let mut q = Queue::new();
        q.push(10);
        q.push(5);
        q.push(1);
        q.push(0);
        assert_eq!(q.pop(), Some(10));
        assert_eq!(q.pop(), Some(5));
        assert_eq!(q.pop(), Some(1));
        assert_eq!(q.pop(), Some(0));
        assert_eq!(q.pop(), None);
    }
}
