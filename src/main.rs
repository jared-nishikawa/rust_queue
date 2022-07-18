pub mod queue;

use queue::Queue;

#[derive(Debug, Clone)]
pub struct SNode {
    val: u64,
    next: Option<Box<SNode>>,
}

pub struct Stack {
    top: Option<SNode>,
}

impl Stack {
    pub fn new() -> Self {
        Stack{top: None}
    }

    pub fn push(&mut self, v: u64) {
        match &self.top {
            None => self.top = Some(SNode{ val: v, next: None}),
            Some(n) => {
                self.top = Some(SNode{ val: v, next: Some(Box::new((*n).clone()))});
            }
        }
    }

    pub fn pop(&mut self) -> Option<u64> {
        match &self.top {
            None => None,
            Some(n) => {
                let v = n.val;
                self.top = match &n.next {
                    Some(s) => Some(*(*s).clone()),
                    None => None,
                };
                Some(v)
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
    /*
    let mut s = Stack::new();
    s.push(1);
    s.push(2);
    s.push(3);
    s.push(4);
    assert_eq!(s.pop(), Some(4));
    assert_eq!(s.pop(), Some(3));
    assert_eq!(s.pop(), Some(2));
    assert_eq!(s.pop(), Some(1));
    assert_eq!(s.pop(), None);
    assert_eq!(s.pop(), None);
    assert_eq!(s.pop(), None);
    */

    let mut q = Queue::new();
    q.push(10);
    q.push(5);
    q.push(1);
    q.push(0);
    println!("{:?}", q.pop());
    println!("{:?}", q.pop());
    println!("{:?}", q.pop());
    println!("{:?}", q.pop());
    println!("{:?}", q.pop());
}
