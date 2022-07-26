mod default_state;
pub use default_state::DefaultState;

use wgpu::{CommandEncoder, Queue, TextureView};
use winit::event::Event;

pub trait State {
    fn resize(
        &mut self,
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        new_size: winit::dpi::PhysicalSize<u32>,
    );
    fn input(&mut self, event: &Event<()>) -> bool;
    fn update(&mut self, queue: &Queue, dt: instant::Duration);
    fn render(
        &self,
        view: &TextureView,
        encoder: &mut CommandEncoder,
    ) -> Result<(), wgpu::SurfaceError>;
}
