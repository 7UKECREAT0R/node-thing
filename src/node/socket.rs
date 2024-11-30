use super::data_model::DataType;

/// A node's socket definition.
pub struct NodeSocket {
	label: String,
	identifier: String,
	direction: SocketDirection,
	preferred_type: DataType
}

impl NodeSocket {
	pub fn new(label: String, identifier: String, direction: SocketDirection, preferred_type: DataType) -> NodeSocket {
		NodeSocket {
			label,
			identifier,
			direction,
			preferred_type
		}
	}
	pub fn label(&self) -> &String {
		&self.label
	}
	pub fn identifier(&self) -> &String {
		&self.identifier
	}
	pub fn direction(&self) -> SocketDirection {
		self.direction // copy
	}
	pub fn preferred_direction(&self) -> DataType {
		self.preferred_type // copy
	}
}

impl Clone for NodeSocket {
	fn clone(&self) -> Self {
		Self {
			label: self.label.clone(),
			identifier: self.identifier.clone(),
			direction: self.direction,
			preferred_type: self.preferred_type
		}
	}
}


/// The direction that a socket is facing (input or output).
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum SocketDirection {
	Input,
	Output
}