use std::sync::Arc;

use wgpu::{Device, util::DeviceExt, Queue};

use super::{Vertex, Renderable, Instance};

pub struct Mesh {
    pub vertex_array: Vec<Vertex>,
    pub index_array: Vec<u16>,
    pub num_indices: u32,
    pub instance_array: Vec<Instance>,
    pub texture_bind_group: Option<Arc<wgpu::BindGroup>>,
    pub vertex_buffer: Option<wgpu::Buffer>,
    pub index_buffer: Option<wgpu::Buffer>,
    pub instance_buffer: Option<wgpu::Buffer>,
}

impl Renderable for Mesh {
    fn initialize(&mut self, device: &Device) {
        self.vertex_buffer = Some(device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&self.vertex_array),
            usage: wgpu::BufferUsages::VERTEX,
        }));

        self.index_buffer = Some(device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&self.index_array),
            usage: wgpu::BufferUsages::INDEX,
        }));
        self.num_indices = self.index_array.len() as u32;

        let instance_data = self.instance_array
            .iter()
            .map(Instance::to_raw)
            .collect::<Vec<_>>();
        self.instance_buffer = Some(device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(&instance_data),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        }));
    }

    fn update_instances(&mut self, queue: &Queue) {
        let instance_data = self
            .instance_array
            .iter()
            .map(Instance::to_raw)
            .collect::<Vec<_>>();
        queue.write_buffer(
            &self.instance_buffer.as_ref().unwrap(),
            0,
            bytemuck::cast_slice(&instance_data),
        );
    }

    fn prepare<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_bind_group(0, &self.texture_bind_group.as_ref().unwrap(), &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.as_ref().unwrap().slice(..));
        render_pass.set_vertex_buffer(1, self.instance_buffer.as_ref().unwrap().slice(..));
        render_pass.set_index_buffer(self.index_buffer.as_ref().unwrap().slice(..), wgpu::IndexFormat::Uint16);
    }

    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.draw_indexed(0..self.num_indices as _, 0, 0..self.instance_array.len() as _);
    }
}