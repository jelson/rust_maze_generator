// Re-export shape implementations from individual modules
mod rect_shape;
mod tri_shape;
mod hex_shape;

pub use rect_shape::RectShape;
pub use tri_shape::TriShape;
pub use hex_shape::HexShape;
