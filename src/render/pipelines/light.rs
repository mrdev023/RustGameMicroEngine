use crate::{
    model::{self, Vertex},
    render, texture,
};

use super::GlobalBindLayout;

pub struct LightPipeline {
    pipeline: wgpu::RenderPipeline,
}

impl LightPipeline {
    pub fn new(
        global_bind_layout: &GlobalBindLayout,
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
    ) -> Self {
        let pipeline = {
            let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Light Pipeline Layout"),
                bind_group_layouts: &[
                    global_bind_layout.get_camera_bind_layout(),
                    global_bind_layout.get_light_bind_layout(),
                ],
                push_constant_ranges: &[],
            });
            let shader = wgpu::ShaderModuleDescriptor {
                label: Some("Light Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("light.wgsl").into()),
            };
            render::create_render_pipeline(
                &device,
                &layout,
                config.format,
                Some(texture::Texture::DEPTH_FORMAT),
                &[model::ModelVertex::desc()],
                shader,
            )
        };

        Self { pipeline }
    }

    pub fn get_pipeline(&self) -> &wgpu::RenderPipeline {
        &self.pipeline
    }
}
