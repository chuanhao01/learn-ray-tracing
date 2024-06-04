use crate::{gpu_buffer, InitConfig, Vec3f};
use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

struct Lambertain {
    albedo: Vec3f,
}
impl Lambertain {
    fn new(albedo: Vec3f) -> Self {
        Self { albedo }
    }
}
impl ScatterMaterial for Lambertain {
    fn get_t(&self) -> ScatterT {
        ScatterT::Lambertain
    }
    fn to_gpu_material(&self) -> gpu_buffer::ScatterMaterial {
        gpu_buffer::ScatterMaterial::new(self.albedo, self.get_t() as u32, 0f32, 0f32)
    }
}

trait ScatterMaterial {
    fn get_t(&self) -> ScatterT;
    fn to_gpu_material(&self) -> gpu_buffer::ScatterMaterial;
}
enum ScatterT {
    Lambertain,
}

struct Diffuse {
    power: f32,
}
impl Diffuse {
    fn new(power: f32) -> Self {
        Self { power }
    }
}
impl EmitMaterial for Diffuse {
    fn get_t(&self) -> EmitT {
        EmitT::Diffuse
    }
    fn to_gpu_material(&self) -> gpu_buffer::EmitMaterial {
        gpu_buffer::EmitMaterial::new(self.get_t() as u32, self.power)
    }
}

trait EmitMaterial {
    fn get_t(&self) -> EmitT;
    fn to_gpu_material(&self) -> gpu_buffer::EmitMaterial;
}
enum EmitT {
    Diffuse,
}

enum Material {
    ScatterMaterial(Box<dyn ScatterMaterial>),
    EmitMaterial(Box<dyn EmitMaterial>),
}
impl Material {
    fn get_t(&self) -> u32 {
        match self {
            Self::ScatterMaterial(_) => 0,
            Self::EmitMaterial(_) => 1,
        }
    }
}

struct Scene {
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

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C, align(16))]
pub struct Uniforms {
    pub look_at: Vec3f,
    pub theta: f32,
    pub look_from: Vec3f,
    pub focal_distance: f32,
    pub v_up: Vec3f,
    pub vp_width: u32,
    pub vp_height: u32,
    pub frame_count: u32,
    pub _padding: [u32; 2],
}
impl Uniforms {
    pub fn from_init_configs(init_configs: InitConfig) -> Self {
        Self {
            vp_width: init_configs.vp_width,
            vp_height: init_configs.vp_height,
            focal_distance: init_configs.camera_focal_distance,
            theta: init_configs.camera_theta,
            frame_count: 0,
            look_at: init_configs.look_at,
            look_from: init_configs.look_from,
            v_up: init_configs.v_up,
            _padding: [0u32; 2],
        }
    }
}

pub struct PathTracer {
    device: wgpu::Device,
    queue: wgpu::Queue,

    uniforms: Uniforms,
    uniform_buffer: wgpu::Buffer,

    scene: Scene,

    display_pipeline: wgpu::RenderPipeline,
    data_bind_group: wgpu::BindGroup,
    radiance_bind_group: [wgpu::BindGroup; 2],
}

impl PathTracer {
    fn generate_scene() -> Scene {
        let mut scene = Scene::new();
        let lambertain_ground = Lambertain::new(Vec3f::new(0.8, 0.8, 0.0));
        let lambertain_red = Lambertain::new(Vec3f::new(0.7, 0.0, 0.0));
        let lambertain_green = Lambertain::new(Vec3f::new(0.0, 0.7, 0.0));
        let lambertain_blue = Lambertain::new(Vec3f::new(0.1, 0.2, 0.5));

        // So the buffer is not empty
        let diffuse = Diffuse::new(5f32);

        let mat_ground_id =
            scene.add_material(Material::ScatterMaterial(Box::new(lambertain_ground)));
        let mat_red_id = scene.add_material(Material::ScatterMaterial(Box::new(lambertain_red)));
        let mat_green_id =
            scene.add_material(Material::ScatterMaterial(Box::new(lambertain_green)));
        let mat_blue_id = scene.add_material(Material::ScatterMaterial(Box::new(lambertain_blue)));
        let diffuse_id = scene.add_material(Material::EmitMaterial(Box::new(diffuse)));

        let floor_sphere =
            gpu_buffer::Sphere::new(Vec3f::new(0.0, -100.5, -1.0), 100.0, mat_ground_id);
        let left_sphere = gpu_buffer::Sphere::new(Vec3f::new(-1.0, 0.0, -1.0), 0.5, mat_red_id);
        let middle_sphere = gpu_buffer::Sphere::new(Vec3f::new(0.0, 0.0, -1.0), 0.5, mat_green_id);
        let right_sphere = gpu_buffer::Sphere::new(Vec3f::new(1.0, 0.0, -1.0), 0.5, mat_blue_id);

        scene.spheres = vec![floor_sphere, left_sphere, middle_sphere, right_sphere];
        scene
    }
    pub fn new(device: wgpu::Device, queue: wgpu::Queue, init_configs: InitConfig) -> Self {
        device.on_uncaptured_error(Box::new(|error| panic!("Aborting due to error, {}", error)));

        // TODO: init GPU resources
        let shader_module = compile_shader_module(&device);
        let (display_pipeline, [data_bind_group_layout, radiance_bind_group_layout]) =
            create_display_pipeline(&device, &shader_module);

        let uniforms = Uniforms::from_init_configs(init_configs);
        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("uniforms"),
            size: std::mem::size_of::<Uniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let scene = Self::generate_scene();

        let radiance_samples =
            create_sample_textures(&device, uniforms.vp_width, uniforms.vp_height);
        let data_bind_group = Self::create_data_bind_group(
            &device,
            data_bind_group_layout,
            &uniform_buffer,
            &scene.get_spheres_buffer(&device),
            &scene.get_materials_buffer(&device),
        );
        let radiance_bind_group = Self::create_radiance_bind_group(
            &device,
            radiance_bind_group_layout,
            &radiance_samples,
        );

        Self {
            device,
            queue,

            uniforms,
            uniform_buffer,

            scene,

            display_pipeline,
            data_bind_group,
            radiance_bind_group,
        }
    }
    pub fn render_frame(&mut self, target: &wgpu::TextureView) {
        // Update to uniforms
        self.uniforms.frame_count += 1;
        self.queue
            .write_buffer(&self.uniform_buffer, 0, bytemuck::bytes_of(&self.uniforms));

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("render frame"),
            });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("display pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
            })],
            ..Default::default()
        });

        render_pass.set_pipeline(&self.display_pipeline);
        render_pass.set_bind_group(0, &self.data_bind_group, &[]);
        render_pass.set_bind_group(
            1,
            &self.radiance_bind_group[(self.uniforms.frame_count % 2) as usize],
            &[],
        );

        // Draw verticies 0-2, 1 instance
        render_pass.draw(0..6, 0..1);

        // Ending the render by dropping the object
        drop(render_pass);

        let command_buffer = encoder.finish();
        self.queue.submit(Some(command_buffer));
    }

    fn create_radiance_bind_group(
        device: &wgpu::Device,
        layout: wgpu::BindGroupLayout,
        textures: &[wgpu::Texture; 2],
    ) -> [wgpu::BindGroup; 2] {
        let views = [
            textures[0].create_view(&wgpu::TextureViewDescriptor {
                ..Default::default()
            }),
            textures[1].create_view(&wgpu::TextureViewDescriptor {
                ..Default::default()
            }),
        ];
        [
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&views[0]),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::TextureView(&views[1]),
                    },
                ],
            }),
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&views[1]),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::TextureView(&views[0]),
                    },
                ],
            }),
        ]
    }

    fn create_data_bind_group(
        device: &wgpu::Device,
        layout: wgpu::BindGroupLayout,
        uniform_buffer: &wgpu::Buffer,
        spheres_buffer: &wgpu::Buffer,
        materials_buffer: &[wgpu::Buffer; 3],
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: uniform_buffer,
                        offset: 0,
                        size: None,
                    }),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: spheres_buffer,
                        offset: 0,
                        size: None,
                    }),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &materials_buffer[0],
                        offset: 0,
                        size: None,
                    }),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &materials_buffer[1],
                        offset: 0,
                        size: None,
                    }),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &materials_buffer[2],
                        offset: 0,
                        size: None,
                    }),
                },
            ],
        })
    }
}

fn compile_shader_module(device: &wgpu::Device) -> wgpu::ShaderModule {
    use std::borrow::Cow;

    let code = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/shaders.wgsl"));
    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(code)),
    })
}

fn create_display_pipeline(
    device: &wgpu::Device,
    shader_module: &wgpu::ShaderModule,
) -> (wgpu::RenderPipeline, [wgpu::BindGroupLayout; 2]) {
    let radiance_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: false },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::StorageTexture {
                        access: wgpu::StorageTextureAccess::WriteOnly,
                        format: wgpu::TextureFormat::Rgba32Float,
                        view_dimension: wgpu::TextureViewDimension::D2,
                    },
                    count: None,
                },
            ],
        });
    let data_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 4,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });
    let display_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("display"),
        layout: Some(
            &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                bind_group_layouts: &[&data_bind_group_layout, &radiance_bind_group_layout],
                ..Default::default()
            }),
        ),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            front_face: wgpu::FrontFace::Ccw,
            polygon_mode: wgpu::PolygonMode::Fill,
            ..Default::default()
        },
        vertex: wgpu::VertexState {
            module: shader_module,
            entry_point: "display_vs",
            buffers: &[],
        },
        fragment: Some(wgpu::FragmentState {
            module: shader_module,
            entry_point: "display_fs",
            targets: &[Some(wgpu::ColorTargetState {
                blend: None,
                format: wgpu::TextureFormat::Bgra8Unorm,
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
    });
    (
        display_pipeline,
        [data_bind_group_layout, radiance_bind_group_layout],
    )
}

// Width and height are needed, to know how big the texture should be and stored data
fn create_sample_textures(device: &wgpu::Device, width: u32, height: u32) -> [wgpu::Texture; 2] {
    let desc = wgpu::TextureDescriptor {
        label: Some("randiance samples"),
        size: wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba32Float,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::STORAGE_BINDING,
        view_formats: &[],
    };
    [device.create_texture(&desc), device.create_texture(&desc)]
}
