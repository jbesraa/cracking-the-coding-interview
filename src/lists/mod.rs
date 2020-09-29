mod node;

use crate::lists::node::{Node, NodeOption};

#[derive(PartialEq, Debug)]
pub struct LinkedList {
	head: NodeOption,
	tail: NodeOption,
	pub length: usize,
}

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
}

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
