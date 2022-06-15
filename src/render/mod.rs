mod vertex;
pub use vertex::Vertex;

mod camera;
pub use camera::{
  Camera, CameraUniform, CameraController
};

mod texture;
pub use texture::Texture;

mod instance;
pub use instance::{
  Instance, InstanceRaw
};
use wgpu::{Device, Queue};

mod mesh;
pub use mesh::Mesh;

pub trait Renderable {
  fn prepare(&mut self, device: &Device);
  fn update_instances(&mut self, device: &Queue);
  fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>);
}