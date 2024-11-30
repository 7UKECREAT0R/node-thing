#[derive(Debug)]
pub enum NodeError {
	WrongDirection(String)
}
impl std::fmt::Display for NodeError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			NodeError::WrongDirection(s) => write!(f, "Wrong node direction: {}", s),
		}
	}
}
impl std::error::Error for NodeError {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		None
	}
}