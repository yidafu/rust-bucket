use std::collections::VecDeque;
use std::collections::LinkedList;

fn main() {
  let mut buf = VecDeque::new();
  buf.push_front(1);
  buf.push_front(2);
  assert_eq!(buf.get(0), Some(&2));

  let mut list = LinkedList::new();

  list.push_back('a');
  list.push_back('b');
  list.push_back('c');
  println!("{:?}", list);
  let c = list.pop_back();
  println!("{:?}", c);
}