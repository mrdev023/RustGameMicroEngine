mod texture;
pub use texture::Texture;
use wgpu::{BindGroup, Device, Queue};

pub struct TextureManager {
    texture_bind_group_layout: wgpu::BindGroupLayout,
}

impl TextureManager {
    pub fn new(device: &Device) -> Self {
        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });
        Self {
            texture_bind_group_layout,
        }
    }

    pub fn create_texture_from_bytes(
        &self,
        device: &Device,
        queue: &Queue,
        bytes: &[u8],
        label: &str,
    ) -> BindGroup {
        let diffuse_texture = Texture::from_bytes(&device, &queue, bytes, label).unwrap();

        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                },
            ],
            label: Some(&format!("diffuse_bind_group_{}", label)),
        })
    }

    pub fn get_texture_bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.texture_bind_group_layout
    }
}
