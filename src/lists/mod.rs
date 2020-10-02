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
			length: 0,
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
				length: 0
			}
		)
	}
}
