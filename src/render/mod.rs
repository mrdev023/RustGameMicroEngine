mod vertex;
pub use vertex::Vertex;

mod camera;
pub use camera::Camera;

mod texture;
pub use texture::{Texture, TextureManager};

mod instance;
pub use instance::{
  Instance, InstanceRaw
};
use wgpu::{Device, Queue};

mod mesh;
pub use mesh::Mesh;

mod window;
pub use window::Window;

mod pipelines;

pub trait Renderable {
  fn initialize(&mut self, device: &Device);
  fn update_instances(&mut self, queue: &Queue);
  fn prepare<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>);
  fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>);
}