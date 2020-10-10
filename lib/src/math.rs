pub type Vector2 = cgmath::Vector2<f32>;
pub type Vector3 = cgmath::Vector3<f32>;
pub type Vector4 = cgmath::Vector4<f32>;
pub type Matrix4 = cgmath::Matrix4<f32>;

pub mod prelude {
    pub use super::{Vector2, Vector3, Vector4, Matrix4};
}