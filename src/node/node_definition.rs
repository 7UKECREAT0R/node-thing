use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};
use super::node_error::NodeError;
use super::socket::{NodeSocket, SocketDirection};
use crate::node::Node;
use ggez::graphics::Color;
use ggez::Context;
use crate::NodeThing;

static _LAST_NODE_DEFINITION_ID: AtomicU64 = AtomicU64::new(0);
fn create_node_definition_id() -> u64 {
	_LAST_NODE_DEFINITION_ID.fetch_add(1, Ordering::SeqCst)
}

pub struct NodeDefinition {
	node_definition_id: u64,
	name: String,
	description: String,
	identifier: String,
	color: Color,
	input_sockets: Vec<NodeSocket>,
	output_sockets: Vec<NodeSocket>,
}

impl NodeDefinition {
	pub fn new(name: impl Into<String>, description: impl Into<String>, identifier: impl Into<String>, color: Color) -> NodeDefinition {
		NodeDefinition {
			node_definition_id: create_node_definition_id(),
			name: name.into(),
			description: description.into(),
			identifier: identifier.into(),
			color,
			input_sockets: Vec::new(),
			output_sockets: Vec::new(),
		}
	}
	pub fn node_definition_id(&self) -> u64 {
		self.node_definition_id
	}
	pub fn name(&self) -> &String {
		&self.name
	}
	pub fn description(&self) -> &String {
		&self.description
	}
	pub fn identifier(&self) -> &String {
		&self.identifier
	}
	pub fn color(&self) -> Color {
		self.color
	}

	pub fn iterate_input_sockets(&self) -> std::slice::Iter<NodeSocket> {
		self.input_sockets.iter()
	}
	pub fn iterate_output_sockets(&self) -> std::slice::Iter<NodeSocket> {
		self.output_sockets.iter()
	}
	pub fn add_input_socket(&mut self, socket: NodeSocket) -> Result<(), NodeError> {
		if socket.direction() != SocketDirection::Input {
			return Err(NodeError::WrongDirection(
				"Got output socket in add_input_socket".to_string(),
			));
		}
		self.input_sockets.push(socket);
		Ok(())
	}
	pub fn add_output_socket(&mut self, socket: NodeSocket) -> Result<(), NodeError> {
		if socket.direction() != SocketDirection::Output {
			return Err(NodeError::WrongDirection(
				"Got input socket in add_output_socket".to_string(),
			));
		}
		self.output_sockets.push(socket);
		Ok(())
	}
	
	/// Creates a `Node` from this `NodeDefinition`.
	pub fn create_node(&self, ctx: &Context) -> Node {
		Node::new_from_definition(ctx, self)
	}
}

impl Default for NodeDefinition {
	fn default() -> Self {
		Self {
			node_definition_id: create_node_definition_id(),
			name: "Default Node".to_string(),
			description: "This node is unimplemented and does literally nothing.".to_string(),
			identifier: "default".to_string(),
			color: Color::WHITE,
			input_sockets: Vec::new(),
			output_sockets: Vec::new(),
		}
	}
}