use crate::{
    model::{self, Vertex},
    render, texture, InstanceRaw,
};

use super::GlobalBindLayout;

pub struct ModelPipeline {
    pipeline: wgpu::RenderPipeline,
}

impl ModelPipeline {
    pub fn new(
        global_bind_layout: &GlobalBindLayout,
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
    ) -> Self {
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[
                    global_bind_layout.get_texture_bind_layout(),
                    global_bind_layout.get_camera_bind_layout(),
                    global_bind_layout.get_light_bind_layout(),
                ],
                push_constant_ranges: &[],
            });

        let pipeline = {
            let shader = wgpu::ShaderModuleDescriptor {
                label: Some("Normal Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("model.wgsl").into()),
            };
            render::create_render_pipeline(
                &device,
                &render_pipeline_layout,
                config.format,
                Some(texture::Texture::DEPTH_FORMAT),
                &[model::ModelVertex::desc(), InstanceRaw::desc()],
                shader,
            )
        };

        Self { pipeline }
    }

    pub fn get_pipeline(&self) -> &wgpu::RenderPipeline {
        &self.pipeline
    }
}
