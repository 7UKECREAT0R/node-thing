use super::{CurrentAction, NodeThing};
use ggez::event::{EventHandler, MouseButton};
use ggez::graphics::{Canvas, Color, DrawParam};
use ggez::input::keyboard::KeyInput;
use ggez::mint::{Point2, Vector2};
use ggez::winit::event::VirtualKeyCode;
use ggez::{Context, GameError, GameResult};

/// The maximum distance, in pixels, between mouse down and mouse up for it to count as a click.
const MAX_DISTANCE_FOR_CLICK: f32 = 2.0;

impl EventHandler for NodeThing {
	fn update(&mut self, _ctx: &mut Context) -> GameResult {
		Ok(())
	}
	fn draw(&mut self, ctx: &mut Context) -> GameResult {
		let mut canvas = Canvas::from_frame(ctx, Color::BLACK);
		
		// draw all nodes
		for node in &self.nodes {
			canvas.draw(node, DrawParam::default()
				.scale(Vector2 { x: self.viewport_scale, y: self.viewport_scale })
			);
		}
		
		canvas.finish(ctx)
	}

	fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) -> GameResult {
		let world_coords = self.window_coordinates_to_world(ctx, x, y);
		let world_x = world_coords.0;
		let world_y = world_coords.1;
		self.mouse_down_x = x;
		self.mouse_down_y = y;
		match button {
			MouseButton::Left => {
				self.left_mouse_down = true;

				let shift = ctx.keyboard.is_key_pressed(VirtualKeyCode::LShift) || ctx.keyboard.is_key_pressed(VirtualKeyCode::RShift);
				if !shift {
					self.nodes.iter_mut()
						.filter(|node| !node.point_inside(world_x, world_y))
						.for_each(|node| node.selected = false);
				}
				
				// select the first node that's hovered, if any
				let mut hovered_nodes = self.nodes_inside_point(world_x, world_y);
				if let Some(hovered_node) = hovered_nodes.get_mut(0) {
					hovered_node.selected = if shift { !hovered_node.selected } else { true };
				} else {
					// ...
				}
			}
			MouseButton::Right => {
				self.right_mouse_down = true;
			}
			MouseButton::Middle => {
				self.current_action = CurrentAction::MovingViewport;
			}
			MouseButton::Other(_) => {}
		}
		Ok(())
	}

	fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) -> Result<(), GameError> {
		let distance_x = (x - self.mouse_down_x).abs();
		let distance_y = (y - self.mouse_down_y).abs();
		let distance = (distance_x * distance_x + distance_y * distance_y).sqrt();
		let was_click = distance <= MAX_DISTANCE_FOR_CLICK;
		
		match button {
			MouseButton::Left => {
				self.left_mouse_down = false;
			}
			MouseButton::Right => {
				self.right_mouse_down = false;
			}
			MouseButton::Middle => {
				if self.current_action == CurrentAction::MovingViewport {
					self.current_action = CurrentAction::None;
				}
			}
			MouseButton::Other(_) => {
				
			}
		}
		Ok(())
	}
	fn mouse_motion_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32, dx: f32, dy: f32) -> GameResult {
		if self.current_action == CurrentAction::MovingViewport {
			self.set_viewport_position(
				self.viewport_x - dx / self.viewport_scale,
				self.viewport_y - dy / self.viewport_scale
			);
			return Ok(())
		}
		
		if self.left_mouse_down {
			// left click dragging, so move selected nodes.
			for node in &mut self.nodes {
				if node.selected {
					node.move_position(dx / self.viewport_scale, dy / self.viewport_scale);
				}
			}
		}
		
		Ok(())
	}
	fn mouse_wheel_event(&mut self, ctx: &mut Context, _x: f32, y: f32) -> GameResult {
		if y.abs() < 0.001 { return Ok(()); }
		const SCROLL_AMOUNT: f32 = 0.1;
		
		let mouse_position = ctx.mouse.position();
		let (mouse_position_world_x, mouse_position_world_y) =
			self.window_coordinates_to_world(ctx, mouse_position.x, mouse_position.y);
		let distance_from_viewport_corner_x = mouse_position_world_x - self.viewport_x;
		let distance_from_viewport_corner_y = mouse_position_world_y - self.viewport_y;
		
		let zoom = (y * -1.0) * SCROLL_AMOUNT + 1.0;
		
		self.set_viewport_position(
			self.viewport_x + distance_from_viewport_corner_x * (y * SCROLL_AMOUNT),
			self.viewport_y + distance_from_viewport_corner_y * (y * SCROLL_AMOUNT)
		);
		self.viewport_scale /= zoom;
		Ok(())
	}
	fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, repeated: bool) -> GameResult {
		if repeated { return Ok(()); }
		
		if let Some(keycode) = input.keycode {
			match keycode {
				VirtualKeyCode::Z => {
					// fit nodes to frame
					self.fit_viewport_to_nodes(ctx);
				}
				_ => {}
			}
		}
		
		Ok(())
	}
}