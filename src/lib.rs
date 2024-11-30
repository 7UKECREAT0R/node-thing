use ggez::Context;
use ggez::graphics::FontData;
use crate::node::Node;
use crate::node::node_definition::NodeDefinition;

pub mod node;
mod event_handler;

pub const ROBOTO: &str = "Roboto";
pub const INTER: &str = "Inter";

pub struct NodeThing {
	node_definitions: Vec<NodeDefinition>,
	nodes: Vec<Node>,
	
	mouse_down_x: f32,
	mouse_down_y: f32,
	left_mouse_down: bool,
	right_mouse_down: bool,
	current_action: CurrentAction,
	viewport_x: f32,
	viewport_y: f32,
	viewport_scale: f32
}
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum CurrentAction {
	/// No action is being taken.
	None,
	/// The user is dragging node(s) using left click.
	MovingNodes,
	/// The user is moving the viewport with the middle mouse button.
	MovingViewport
}
impl NodeThing {
	pub fn new(ctx: &mut Context) -> NodeThing {
		ctx.gfx.add_font(ROBOTO, FontData::from_path(ctx, "/fonts/roboto.ttf").expect("Failed to find './resources/fonts/roboto.ttf'"));
		ctx.gfx.add_font(INTER, FontData::from_path(ctx, "/fonts/inter.ttf").expect("Failed to find './resources/fonts/inter.ttf'"));
		NodeThing {
			node_definitions: Vec::new(),
			nodes: Vec::new(),
			mouse_down_x: 0.0,
			mouse_down_y: 0.0,
			left_mouse_down: false,
			right_mouse_down: false,
			current_action: CurrentAction::None,
			viewport_x: 0.0,
			viewport_y: 0.0,
			viewport_scale: 1.0,
		}
	}
	pub fn add_node(&mut self, mut node: Node) {
		node.set_base_position(self.viewport_x, self.viewport_y);
		self.nodes.push(node);
	}
	pub fn add_node_definition(&mut self, node_definition: NodeDefinition) {
		self.node_definitions.push(node_definition);
	}
	pub fn nodes_inside_point(&mut self, x: f32, y: f32) -> Vec<&mut Node> {
		self.nodes.iter_mut().filter(|node| node.point_inside(x, y)).collect()
	}
	pub fn set_viewport_position(&mut self, x: f32, y: f32) {
		self.viewport_x = x;
		self.viewport_y = y;
		
		// update offsets in child nodes
		self.nodes.iter_mut().for_each(|node| node.set_base_position(self.viewport_x, self.viewport_y));
	}

	fn fit_viewport_to_nodes(&mut self, ctx: &Context) {
		if self.nodes.len() > 0 {
			const FIT_PADDING: f32 = 0.1;
			let mut left = 9999999.0;
			let mut top = 9999999.0;
			let mut right = -9999999.0;
			let mut bottom = -9999999.0;
			for node in &self.nodes {
				if node.get_left() < left { left = node.get_left(); }
				if node.get_top() < top { top = node.get_top(); }
				if node.get_right() > right { right = node.get_right(); }
				if node.get_bottom() > bottom { bottom = node.get_bottom(); }
			}
			let viewport_width = ctx.gfx.size().0;
			let viewport_height = ctx.gfx.size().1;
			let conversion_x = (right - left) / viewport_width;
			let conversion_y = (bottom - top) / viewport_height;
			let larger_scale = f32::max(conversion_x, conversion_y);
			self.viewport_scale = 1.0 / (larger_scale + FIT_PADDING * 2.0);
			self.set_viewport_position(
				left - FIT_PADDING * ctx.gfx.size().0,
				top - FIT_PADDING * ctx.gfx.size().1
			);
		}
	}
	pub fn window_coordinates_to_world(&self, ctx: &Context, x: f32, y: f32) -> (f32, f32) {
		let window_width = ctx.gfx.size().0;
		let window_height = ctx.gfx.size().1;
		let viewport_width = window_width / self.viewport_scale;
		let viewport_height = window_height / self.viewport_scale;
		let conversion_x = viewport_width / window_width;
		let conversion_y = viewport_height / window_height;
		(x * conversion_x + self.viewport_x, y * conversion_y + self.viewport_y)
	}
}