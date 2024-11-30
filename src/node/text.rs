use ggez::context::Has;
use ggez::graphics::{Canvas, Color, DrawParam, Drawable, GraphicsContext, PxScale, Rect, TextFragment};
use ggez::mint::Point2;

pub struct Text {
	internal_text: Option<TextFragment>,
	content: String,
	base_x: f32, // set by parent
	base_y: f32, // set by parent
	x: f32,
	y: f32,
	font: Option<String>,
	size: f32,
	color: Color,
	bounds: Point2<f32>
}

impl Text {
	pub fn new(content: impl Into<String>, font: Option<impl Into<String>>,
	           size: f32, color: Color, bounds: Point2<f32>) -> Self {
		Text {
			internal_text: None,
			base_x: 0.0,
			base_y: 0.0,
			x: 0.0,
			y: 0.0,
			content: content.into(),
			font: match font {
				None => { None }
				Some(f) => { Some(f.into()) }
			},
			size,
			color,
			bounds
		}.generate_internal_text()
	}
	pub fn set_base_position(&mut self, x: f32, y: f32) {
		self.base_x = x;
		self.base_y = y;
	}
	pub fn set_position(&mut self, x: f32, y: f32) {
		self.x = x;
		self.y = y;
	}

	pub fn with_base_position(mut self, x: f32, y: f32) -> Self {
		self.base_x = x;
		self.base_y = y;
		self
	}
	pub fn with_position(mut self, x: f32, y: f32) -> Self {
		self.x = x;
		self.y = y;
		self
	}
	fn generate_internal_text(mut self) -> Self {
		if let None = self.internal_text {
			self.internal_text = Some(TextFragment {
				text: self.content.clone(),
				font: Some(self.font.clone().unwrap_or("LiberationMono-Regular".to_string())),
				scale: Some(PxScale::from(self.size)),
				color: Some(self.color),
			});
		}
		self
	}
}

impl Drawable for Text {
	fn draw(&self, canvas: &mut Canvas, param: impl Into<DrawParam>) {
		let draw_param = param.into()
			.color(self.color)
			.offset(Point2 {
				x: self.base_x - self.x,
				y: self.base_y - self.y
			});
		
		if let Some(text) = &self.internal_text {
			let mut t = ggez::graphics::Text::new(text.clone());
			t.set_bounds(self.bounds.clone());
			canvas.draw(&t, draw_param);
		}
	}
	fn dimensions(&self, gfx: &impl Has<GraphicsContext>) -> Option<Rect> {
		match &self.internal_text {
			None => { None }
			Some(i) => {
				None
			}
		}
	}
}