mod renderer;

pub use renderer::Renderer;

mod default_state;
pub use default_state::DefaultState;

use wgpu::{TextureView, CommandEncoder};
use winit::event::WindowEvent;

pub trait State {
  fn new(renderer: &Renderer) -> Self where Self: Sized;
  fn resize(&mut self, renderer: &Renderer, new_size: winit::dpi::PhysicalSize<u32>);
  fn input(&mut self, renderer: &Renderer, event: &WindowEvent) -> bool;
  fn update(&mut self, renderer: &Renderer, dt: instant::Duration);
  fn render(&mut self, renderer: &Renderer, view: &TextureView, encoder: &mut CommandEncoder) -> Result<(), wgpu::SurfaceError>;
}