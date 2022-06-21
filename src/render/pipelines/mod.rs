mod light;
mod model;
pub mod utils;

pub use light::LightPipeline;
pub use model::ModelPipeline;

pub struct GlobalBindLayout {
    texture: wgpu::BindGroupLayout,
    light: wgpu::BindGroupLayout,
    camera: wgpu::BindGroupLayout,
}

impl GlobalBindLayout {
    pub fn new(device: &wgpu::Device) -> Self {
        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 3,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("camera_bind_group_layout"),
            });

        let light_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: None,
            });

        Self {
            texture: texture_bind_group_layout,
            light: light_bind_group_layout,
            camera: camera_bind_group_layout,
        }
    }

    pub fn get_texture_bind_layout(&self) -> &wgpu::BindGroupLayout {
        &self.texture
    }

    pub fn get_light_bind_layout(&self) -> &wgpu::BindGroupLayout {
        &self.light
    }

    pub fn get_camera_bind_layout(&self) -> &wgpu::BindGroupLayout {
        &self.camera
    }
}

pub struct Pipelines {
    render: model::ModelPipeline,
    light: light::LightPipeline,
}

impl Pipelines {
    pub fn new(
        global_bind_layout: &GlobalBindLayout,
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
    ) -> Self {
        Self {
            render: model::ModelPipeline::new(global_bind_layout, device, config),
            light: light::LightPipeline::new(global_bind_layout, device, config),
        }
    }

    pub fn get_render_pipeline(&self) -> &wgpu::RenderPipeline {
        self.render.get_pipeline()
    }

    pub fn get_light_pipeline(&self) -> &wgpu::RenderPipeline {
        self.light.get_pipeline()
    }
}
