use crate::node::Node;

#[derive(Copy, Clone)]
pub struct NodeWrapper<'a> {
	ptr: &'a mut Node,
}

impl<'a> NodeWrapper<'a> {

	pub fn new(pointer: &'a mut Node) -> NodeWrapper<'a> {
		NodeWrapper { ptr: pointer, }
	}

	pub fn get_ptr(&self) {
		self.ptr
	}

}
