use std::sync::Arc;

use cgmath::prelude::*;

use crate::render::{Mesh, Renderable, Vertex, Instance};

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-0.0868241, 0.49240386, 0.0],
        tex_coords: [0.4131759, 0.00759614],
    }, // A
    Vertex {
        position: [-0.49513406, 0.06958647, 0.0],
        tex_coords: [0.0048659444, 0.43041354],
    }, // B
    Vertex {
        position: [-0.21918549, -0.44939706, 0.0],
        tex_coords: [0.28081453, 0.949397],
    }, // C
    Vertex {
        position: [0.35966998, -0.3473291, 0.0],
        tex_coords: [0.85967, 0.84732914],
    }, // D
    Vertex {
        position: [0.44147372, 0.2347359, 0.0],
        tex_coords: [0.9414737, 0.2652641],
    }, // E
];

const INDICES: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4];

const NUM_INSTANCES_PER_ROW: u32 = 10;
const INSTANCE_DISPLACEMENT: cgmath::Vector3<f32> = cgmath::Vector3::new(
    NUM_INSTANCES_PER_ROW as f32 * 0.5,
    0.0,
    NUM_INSTANCES_PER_ROW as f32 * 0.5,
);

const FRAME_TIME: f32 = 1.0 / 60.0;
const ROTATION_SPEED: f32 = std::f32::consts::PI * FRAME_TIME * 0.5;

pub struct DefaultMesh {
  mesh: Mesh,
  toggle: bool,
  texture1_bind_group: Arc<wgpu::BindGroup>,
  texture2_bind_group: Arc<wgpu::BindGroup>,
}

impl DefaultMesh {
  pub fn new(texture1_bind_group: wgpu::BindGroup, texture2_bind_group: wgpu::BindGroup) -> Self {
    let texture1_bind_group = Arc::new(texture1_bind_group);
    let texture2_bind_group = Arc::new(texture2_bind_group);

    let instances = (0..NUM_INSTANCES_PER_ROW)
            .flat_map(|z| {
                (0..NUM_INSTANCES_PER_ROW).map(move |x| {
                    let position = cgmath::Vector3 {
                        x: x as f32,
                        y: 0.0,
                        z: z as f32,
                    } - INSTANCE_DISPLACEMENT;

                    let rotation = if position.is_zero() {
                        // this is needed so an object at (0, 0, 0) won't get scaled to zero
                        // as Quaternions can effect scale if they're not created correctly
                        cgmath::Quaternion::from_axis_angle(
                            cgmath::Vector3::unit_z(),
                            cgmath::Deg(0.0),
                        )
                    } else {
                        cgmath::Quaternion::from_axis_angle(position.normalize(), cgmath::Deg(45.0))
                    };

                    Instance { position, rotation }
                })
            })
            .collect::<Vec<_>>();

    let mesh = Mesh {
      vertex_array: VERTICES.to_vec(),
      index_array: INDICES.to_vec(),
      num_indices: INDICES.len() as u32,
      instance_array: instances,
      texture_bind_group: Some(texture1_bind_group.clone()),
      vertex_buffer: None,
      index_buffer: None,
      instance_buffer: None,
    };
    DefaultMesh {
      mesh,
      toggle: false,
      texture1_bind_group,
      texture2_bind_group,
    }
  }

  pub fn toggle(&mut self, toggle: bool) {
    self.toggle = toggle;
    if !self.toggle {
      self.mesh.texture_bind_group = Some(self.texture1_bind_group.clone());
    } else {
      self.mesh.texture_bind_group = Some(self.texture2_bind_group.clone());
    }
  }
}

impl Renderable for DefaultMesh {
  fn prepare(&mut self, device: &wgpu::Device) {
    self.mesh.prepare(device);
  }

  fn update_instances(&mut self, device: &wgpu::Queue) {
    for instance in self.mesh.instance_array.iter_mut() {
      let amount = cgmath::Quaternion::from_angle_y(cgmath::Rad(ROTATION_SPEED));
      let current = instance.rotation;
      instance.rotation = amount * current;
    }
    self.mesh.update_instances(device);
  }

  fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
    self.mesh.render(render_pass);
  }
}