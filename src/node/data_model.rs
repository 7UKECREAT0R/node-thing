use std::fmt::{Display, Formatter};
use ggez::graphics::Color;

#[derive(Copy, Clone, Debug)]
pub enum DataType {
	Integer,
	Float,
	String,
	Boolean
}

impl DataType {
	/// Returns the human-readable name for this data type.
	pub fn get_type_name(&self) -> &str {
		match self {
			DataType::Integer => "Integer",
			DataType::Float => "Float",
			DataType::String => "String",
			DataType::Boolean => "Boolean",
		}
	}
	/// Returns the graphics color to represent this type.
	pub fn get_type_color(&self) -> Color {
		match self {
			DataType::Integer => Color::new(0.3, 0.86, 0.66, 1.0),
			DataType::Float => Color::new(0.3, 0.78, 0.86, 1.0),
			DataType::String => Color::new(0.92, 0.83, 0.36, 1.0),
			DataType::Boolean => Color::new(0.57, 0.36, 0.92, 1.0),
		}
	}
}
impl Display for DataType {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.write_str(self.get_type_name())
	}
}