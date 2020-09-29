use std::cell::RefCell;
use std::rc::Rc;

// ** Node Implementaion **

// types
type NodeRef = Rc<RefCell<Node>>;
type NodeOption = Option<NodeRef>;

#[derive(PartialEq, Debug)]
pub struct Node {
	data: String,
	next: NodeOption,
}

impl Node {
	pub fn new(text: String) -> NodeRef {
		Rc::new(RefCell::new(Node {
			data: text,
			next: None,
		}))
	}
}

impl Drop for Node {
	fn drop(&mut self) {
		println!("Node with this data -> '{}' just dropped", self.data);
	}
}

mod tests {

	use super::*;

	#[test]
	fn test_new_node() {
		let node = Node::new("node_1".to_string());
		assert_eq!(
			node,
			Rc::new(RefCell::new(Node {
				data: "node_1".to_string(),
				next: None
			}))
		)
	}
}
