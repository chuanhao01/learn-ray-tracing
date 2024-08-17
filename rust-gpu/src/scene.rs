use crate::{gpu_buffer, materials::Material};
use wgpu::util::DeviceExt;

pub struct Scene {
    pub spheres: Vec<gpu_buffer::Sphere>,
    materials: Vec<Material>,
}
impl Scene {
    pub fn new() -> Self {
        Self {
            spheres: Vec::new(),
            materials: Vec::new(),
        }
    }
    /// Use to add materials to be used by objects in the scene
    /// Returns idx to be used on objects
    pub fn add_material(&mut self, material: Material) -> u32 {
        self.materials.push(material);
        self.materials.len() as u32 - 1
    }

    pub fn get_spheres_buffer(&self, device: &wgpu::Device) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            contents: bytemuck::cast_slice(self.spheres.as_slice()),
            label: Some("Sphere Storage Buffer"),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        })
    }

    // Returns, [materials_buffer, scatter_material_buffer, emit_material_buffer]
    pub fn get_materials_buffer(&self, device: &wgpu::Device) -> [wgpu::Buffer; 3] {
        let mut gpu_materials = Vec::new();
        let mut gpu_scatter_materials = Vec::new();
        let mut gpu_emit_materials = Vec::new();

        for material in &self.materials {
            match material {
                Material::ScatterMaterial(scatter_material) => {
                    gpu_scatter_materials.push(scatter_material.to_gpu_material());
                    gpu_materials.push(gpu_buffer::Material::new(
                        material.get_t(),
                        gpu_scatter_materials.len() as u32 - 1,
                        0,
                    ));
                }
                Material::EmitMaterial(emit_material) => {
                    gpu_emit_materials.push(emit_material.to_gpu_material());
                    gpu_materials.push(gpu_buffer::Material::new(
                        material.get_t(),
                        gpu_emit_materials.len() as u32 - 1,
                        0,
                    ));
                }
            }
        }
        [
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                contents: bytemuck::cast_slice(gpu_materials.as_slice()),
                label: Some("Materials Storage Buffer"),
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            }),
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                contents: bytemuck::cast_slice(gpu_scatter_materials.as_slice()),
                label: Some("Scatter Materials Storage Buffer"),
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            }),
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                contents: bytemuck::cast_slice(gpu_emit_materials.as_slice()),
                label: Some("Emit Materials Storage Buffer"),
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            }),
        ]
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}
