use bytemuck::{Pod, Zeroable};
pub struct CameraParams {
    pub width: u32,
    pub aspect_raio: f32,
    pub focal_distance: f32,
}
impl Default for CameraParams {
    fn default() -> Self {
        Self {
            width: 800,
            aspect_raio: 16.0 / 9.0,
            focal_distance: 1.0,
        }
    }
}

#[allow(dead_code)]
pub struct CameraConfig {
    pub width: u32,
    pub height: u32,
    pub aspect_raio: f32,
    pub focal_distance: f32,
}
impl CameraConfig {
    pub fn new(camera_params: CameraParams) -> Self {
        Self {
            width: camera_params.width,
            height: (camera_params.width as f32 / camera_params.aspect_raio) as u32,
            aspect_raio: camera_params.aspect_raio,
            focal_distance: camera_params.focal_distance,
        }
    }
}

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct Uniforms {
    pub vp_width: u32,
    pub vp_height: u32,
    pub focal_distance: f32,
    pub frame_count: u32,
}
impl Uniforms {
    pub fn from_params(camera_params: CameraConfig) -> Self {
        Self {
            vp_width: camera_params.width,
            vp_height: camera_params.height,
            focal_distance: camera_params.focal_distance,
            frame_count: 0,
        }
    }
}

pub struct PathTracer {
    device: wgpu::Device,
    queue: wgpu::Queue,

    uniforms: Uniforms,
    uniform_buffer: wgpu::Buffer,

    display_pipeline: wgpu::RenderPipeline,
    display_bind_group: [wgpu::BindGroup; 2],
}

impl PathTracer {
    pub fn new(device: wgpu::Device, queue: wgpu::Queue, uniforms: Uniforms) -> Self {
        device.on_uncaptured_error(Box::new(|error| panic!("Aborting due to error, {}", error)));

        // TODO: init GPU resources
        let shader_module = compile_shader_module(&device);
        let (display_pipeline, display_layout) = create_display_pipeline(&device, &shader_module);

        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("uniforms"),
            size: std::mem::size_of::<Uniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let radiance_samples =
            create_sample_textures(&device, uniforms.vp_width, uniforms.vp_height);
        let display_bind_group = Self::create_display_bind_group(
            &device,
            display_layout,
            &radiance_samples,
            &uniform_buffer,
        );

        Self {
            device,
            queue,

            uniforms,
            uniform_buffer,

            display_pipeline,
            display_bind_group,
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
        render_pass.set_bind_group(
            0,
            &self.display_bind_group[(self.uniforms.frame_count % 2) as usize],
            &[],
        );

        // Draw verticies 0-2, 1 instance
        render_pass.draw(0..6, 0..1);

        // Ending the render by dropping the object
        drop(render_pass);

        let command_buffer = encoder.finish();
        self.queue.submit(Some(command_buffer));
    }

    fn create_display_bind_group(
        device: &wgpu::Device,
        layout: wgpu::BindGroupLayout,
        textures: &[wgpu::Texture; 2],
        uniform_buffer: &wgpu::Buffer,
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
                        resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                            buffer: uniform_buffer,
                            offset: 0,
                            size: None,
                        }),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::TextureView(&views[0]),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
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
                        resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                            buffer: uniform_buffer,
                            offset: 0,
                            size: None,
                        }),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::TextureView(&views[1]),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::TextureView(&views[0]),
                    },
                ],
            }),
        ]
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
) -> (wgpu::RenderPipeline, wgpu::BindGroupLayout) {
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: false },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
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
    (
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("display"),
            layout: Some(
                &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    bind_group_layouts: &[&bind_group_layout],
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
        }),
        bind_group_layout,
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
