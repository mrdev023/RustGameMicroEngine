pub mod utils;

pub struct GlobalBindLayout {
    texture: wgpu::BindGroupLayout,
    light: wgpu::BindGroupLayout,
    camera: wgpu::BindGroupLayout
}