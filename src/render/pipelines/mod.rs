use wgpu::{Device, Queue};

use super::Renderable;

pub trait Processable {
    fn initialize(&mut self, device: &Device, queue: &Queue, renderable_entities: Vec<Box<dyn Renderable>>);
    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>, renderable_entities: Vec<Box<dyn Renderable>>);
    fn render(&mut self, renderable_entities: Vec<Box<dyn Renderable>>) -> Result<(), wgpu::SurfaceError>;
}