mod renderer;

pub use renderer::Renderer;
use wgpu::{TextureView, CommandEncoder};
use winit::event::WindowEvent;

pub trait State {
  fn new(renderer: &Renderer) -> Self where Self: Sized;
  fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>);
  fn input(&mut self, event: &WindowEvent) -> bool;
  fn update(&mut self, dt: instant::Duration);
  fn render(&mut self, view: &TextureView, encoder: &mut CommandEncoder) -> Result<(), wgpu::SurfaceError>;
}