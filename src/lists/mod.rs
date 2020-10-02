mod node;

use crate::lists::node::{Node, NodeOption};
use std::rc::Rc;

#[derive(PartialEq, Debug)]
pub struct LinkedList {
	head: NodeOption,
	tail: NodeOption,
	pub length: usize,
}

#[allow(dead_code)]
impl LinkedList {
	pub fn new_empty() -> Self {
		LinkedList {
			head: None,
			tail: None,
			length: 0,
		}
	}

	pub fn new(text: String) -> Self {
		let new_head = Node::new(text);

		LinkedList {
			head: Some(new_head),
			tail: None,
			length: 1,
		}
	}

	pub fn append_start(&mut self, text: String) {
		let new_head = Node::new(text);
		if let Some(old_head) = self.head.take() {
			new_head.borrow_mut().next = Some(Rc::clone(&old_head));
			if self.tail.is_none() {
				self.tail = Some(Rc::clone(&old_head))
			}
		}

		self.head = Some(new_head);
		self.length += 1;
	}
}

#[allow(unused_imports)]
mod tests {
	use super::*;
	use std::cell::RefCell;

	#[test]
	fn test_new_empty_list() {
		let list = LinkedList::new_empty();

		assert_eq!(
			list,
			LinkedList {
				head: None,
				tail: None,
				length: 0
			}
		)
	}

	#[test]
	fn test_new_list() {
		let list = LinkedList::new("node_1".to_string());

		assert_eq!(
			list,
			LinkedList {
				head: Some(Node::new("node_1".to_string())),
				tail: None,
				length: 1
			}
		)
	}

	#[test]
	fn test_append_start() {
		let s = "tail".to_string();
		let c = "head".to_string();

		let tail = Node::new(s.clone());

		let head = Node {
			data: c.clone(),
			next: Some(Rc::clone(&tail)),
		};

		let list = LinkedList {
			head: Some(Rc::new(RefCell::new(head))),
			tail: Some(tail),
			length: 2,
		};

		let mut l_list = LinkedList::new_empty();
		l_list.append_start(s);
		l_list.append_start(c);
		assert_eq!(l_list, list);
	}
}
