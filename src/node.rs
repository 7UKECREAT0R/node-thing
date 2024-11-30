use crate::node::node_definition::NodeDefinition;
use crate::node::socket::{NodeSocket, SocketDirection};
use ggez::context::Has;
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Drawable, GraphicsContext, Mesh, Rect, StrokeOptions};
use ggez::{Context, GameResult};
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};
use ggez::mint::Point2;
use text::Text;
use crate::NodeThing;

static _LAST_NODE_ID: AtomicU64 = AtomicU64::new(0);
fn create_node_id() -> u64 {
	_LAST_NODE_ID.fetch_add(1, Ordering::SeqCst)
}

const NODE_WIDTH: f32 = 400.0;
const NODE_HEIGHT: f32 = 200.0;
const NODE_TITLE_BAR_HEIGHT: f32 = 48.0;
const NODE_SOCKET_HEIGHT: f32 = 32.0;
const NODE_EDGE_PADDING: f32 = 30.0;
const NODE_OUTLINE_WIDTH: f32 = 2.0;
const NODE_OUTLINE_ROUNDING: f32 = 4.0;

const NODE_BACKGROUND_COLOR: Color = Color::new(0.1, 0.1, 0.1, 1.0);
const NODE_OUTLINE_COLOR: Color = Color::WHITE;

pub mod data_model;
pub mod node_error;
pub mod node_definition;
pub mod socket;
mod text;

pub struct Node {
	node_id: u64,
	node_definition_id: Option<u64>,
	name: String,
	identifier: String,
	description: String,
	color: Color,
	inputs: Vec<(NodeSocket, Option<Rc<Node>>)>,
	outputs: Vec<(NodeSocket, Option<Rc<Node>>)>,
	base_x: f32,
	base_y: f32,
	pub x: f32,
	pub y: f32,
	width: f32,
	height: f32,
	pub selected: bool,
	
	/// pre-made drawables for drawing this node to the screen
	meshes: Vec<Mesh>,
	texts: Vec<Text>,
	outline: Option<Mesh>,
}

impl Node {
	pub fn new(name: impl Into<String>, identifier: impl Into<String>, description: impl Into<String>, color: Color) -> Node {
		Self {
			node_id: create_node_id(),
			node_definition_id: None,
			
			name: name.into(),
			identifier: identifier.into(),
			description: description.into(),
			color,
			inputs: Vec::new(),
			outputs: Vec::new(),
			base_x: 0.0,
			base_y: 0.0,
			x: 0.0,
			y: 0.0,
			width: NODE_WIDTH,
			height: NODE_HEIGHT + NODE_EDGE_PADDING,
			selected: false,
			meshes: Vec::new(),
			texts: Vec::new(),
			outline: None
		}
	}
	/// Creates a new `Node` from a `NodeDefinition`.
	pub fn new_from_definition(ctx: &Context, definition: &NodeDefinition) -> Node {
		let mut node = Self {
			node_id: create_node_id(),
			node_definition_id: Some(definition.node_definition_id()),
			
			name: definition.name().clone(),
			identifier: definition.identifier().clone(),
			description: definition.description().clone(),
			color: definition.color(),
			inputs: Vec::new(),
			outputs: Vec::new(),
			base_x: 0.0,
			base_y: 0.0,
			x: 0.0,
			y: 0.0,
			width: NODE_WIDTH,
			height: NODE_HEIGHT + NODE_EDGE_PADDING,
			selected: false,
			meshes: Vec::new(),
			texts: Vec::new(),
			outline: None
		};
		definition.iterate_input_sockets().for_each(|input_socket|
			node.create_socket(input_socket.clone())
		);
		definition.iterate_output_sockets().for_each(|output_socket|
			node.create_socket(output_socket.clone())
		);
		node.recompute_size(ctx).unwrap(); // error prone woo!
		node
	}
	
	pub fn point_inside(&self, x: f32, y: f32) -> bool {
		x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
	}
	pub fn get_bounds(&self) -> Rect {
		Rect {
			x: 0.0,
			y: 0.0,
			w: self.width,
			h: self.height,
		}
	}
	pub fn get_min_height(&self) -> f32 {
		let mut base: f32 = NODE_HEIGHT + NODE_EDGE_PADDING;
		base += self.inputs.len() as f32 * NODE_SOCKET_HEIGHT;
		base += self.outputs.len() as f32 * NODE_SOCKET_HEIGHT;
		base
	}
	pub fn get_left(&self) -> f32 { self.x }
	pub fn get_top(&self) -> f32 { self.y }
	pub fn get_right(&self) -> f32 { self.x + self.width }
	pub fn get_bottom(&self) -> f32 { self.y + self.height }
	

	pub fn invalidate_mesh(&mut self) {
		self.meshes.clear();
		self.texts.clear();
		self.outline = None;
	}
	pub fn generate_mesh(&mut self, ctx: &Context) -> GameResult {
		if self.meshes.len() < 1 && self.texts.len() < 1 {
			let bounds = self.get_bounds();
			
			// the main node rectangle
			self.meshes.push(Mesh::new_rectangle(ctx, DrawMode::fill(), bounds, NODE_BACKGROUND_COLOR)?);
			
			// the title bar
			let title_bar_dimensions = Rect::new(bounds.x, bounds.y, bounds.w, NODE_TITLE_BAR_HEIGHT);
			self.meshes.push(Mesh::new_rectangle(ctx, DrawMode::fill(), title_bar_dimensions, self.color)?);
			// the title
			self.texts.push(
				Text::new(self.name.clone(), Some(crate::ROBOTO),
				          32.0,
				          Color::BLACK,
				          Point2 { x: title_bar_dimensions.w, y: f32::INFINITY },
				).with_base_position(self.x, self.y).with_position(10.0, 10.0)
			);
		}
		if let None = self.outline {
			self.outline = Some(Mesh::new_rounded_rectangle(ctx,
			                                                DrawMode::Stroke(StrokeOptions::default().with_line_width(NODE_OUTLINE_WIDTH)),
			                                                self.get_bounds(),
			                                                NODE_OUTLINE_ROUNDING,
			                                                NODE_OUTLINE_COLOR)?);
		}
		Ok(())
	}
	/// Creates a socket on this node from the given NodeSocket definition.
	/// Note that this moves the NodeSocket, so you should probably `clone` it if it's coming
	/// from a NodeDefinition.
	/// 
	/// You should call `recompute_size(&mut self, &Context)` after this.
	pub fn create_socket(&mut self, socket: NodeSocket) {
		match socket.direction() {
			SocketDirection::Input => self.inputs.push((socket, None)),
			SocketDirection::Output => self.outputs.push((socket, None)),
		}
	}
	fn recompute_size(&mut self, ctx: &Context) -> GameResult {
		let width = NODE_WIDTH;
		let height = self.get_min_height();
		self.set_size(ctx, width, height)
	}
	fn recompute_text_offsets(&mut self) {
		for text in &mut self.texts {
			text.set_base_position(self.base_x - self.x, self.base_y - self.y);
		}
	}
	
	pub fn move_position(&mut self, dx: f32, dy: f32) {
		self.x += dx;
		self.y += dy;
		self.recompute_text_offsets();
	}
	pub fn set_position(&mut self, x: f32, y: f32) {
		self.x = x;
		self.y = y;
		self.recompute_text_offsets();
	}
	pub fn set_base_position(&mut self, base_x: f32, base_y: f32) {
		self.base_x = base_x;
		self.base_y = base_y;
		self.recompute_text_offsets();
	}
	pub fn set_size(&mut self, ctx: &Context, width: f32, height: f32) -> GameResult {
		self.width = f32::max(NODE_WIDTH, width);
		self.height = f32::max(self.get_min_height(), height);
		self.invalidate_mesh();
		self.generate_mesh(ctx)
	}
	pub fn set_position_and_size(&mut self, ctx: &Context, x: f32, y: f32, width: f32, height: f32) -> GameResult {
		self.x = x;
		self.y = y;
		self.width = f32::max(NODE_WIDTH, width);
		self.height = f32::max(self.get_min_height(), height);
		self.recompute_text_offsets();
		self.invalidate_mesh();
		self.generate_mesh(ctx)
		
	}
}

impl Display for Node {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!(
			"Node '{}': {} inputs ({} connected), {} outputs ({} connected)",
			self.identifier,
			self.inputs.len(),
			self.inputs.iter().filter(|(_, node)| node.is_some()).count(),
			self.outputs.len(),
			self.outputs.iter().filter(|(_, node)| node.is_some()).count()
		))
	}
}
impl Drawable for Node {
	fn draw(&self, canvas: &mut Canvas, param: impl Into<DrawParam>) {
		let draw_param = param.into()
			.offset(Point2 { x: self.base_x - self.x, y: self.base_y - self.y });
		
		// draw meshes to canvas
		self.meshes.iter().for_each(|mesh| canvas.draw(mesh, draw_param));
		
		// draw texts to canvas, on top of everything else
		self.texts.iter().for_each(|text| canvas.draw(text, draw_param));
		
		// draw outline if the node is selected
		if self.selected {
			if let Some(mesh) = &self.outline {
				canvas.draw(mesh, draw_param);
			}
		}
	}
	fn dimensions(&self, _gfx: &impl Has<GraphicsContext>) -> Option<Rect> {
		Some(self.get_bounds())
	}
}
impl PartialEq for Node {
	fn eq(&self, other: &Self) -> bool {
		self.node_id.eq(&other.node_id)
	}
	fn ne(&self, other: &Self) -> bool {
		self.node_id.ne(&other.node_id)
	}
}
impl Eq for Node {}