// Originally written in 2023 by Arman Uguray <arman.uguray@gmail.com>
// SPDX-License-Identifier: CC-BY-4.0

use gpu_path_tracing::{
    render::{CameraConfig, CameraParams, Uniforms},
    PathTracer,
};
use {
    anyhow::{Context, Result},
    winit::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::{Window, WindowBuilder},
    },
};

// Assign the appropriate window size in terms of physical pixels based on your display DPI.
const WIDTH: u32 = 1500;

#[pollster::main]
async fn main() -> Result<()> {
    let camera_params = CameraParams {
        width: WIDTH,
        ..Default::default()
    };
    let camera_config = CameraConfig::new(camera_params);

    let event_loop = EventLoop::new();
    let window_size = winit::dpi::PhysicalSize::new(camera_config.width, camera_config.height);
    let window = WindowBuilder::new()
        .with_inner_size(window_size)
        .with_resizable(false)
        .with_title("GPU Path Tracer".to_string())
        .build(&event_loop)?;

    let (device, queue, surface) = connect_to_gpu(&window).await?;
    let uniforms = Uniforms::from_params(camera_config);
    let mut renderer = PathTracer::new(device, queue, uniforms);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent { event, .. } => {
                if event == WindowEvent::CloseRequested {
                    *control_flow = ControlFlow::Exit;
                }
            }
            Event::RedrawRequested(_) => {
                // Wait for the next available frame buffer.
                let frame: wgpu::SurfaceTexture = surface
                    .get_current_texture()
                    .expect("failed to get current texture");

                // TODO: draw frame
                let render_target = frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());
                renderer.render_frame(&render_target);

                frame.present();
            }
            Event::MainEventsCleared => {
                // draw repeatedly
                window.request_redraw();
            }
            _ => (),
        }
    });
}

async fn connect_to_gpu(window: &Window) -> Result<(wgpu::Device, wgpu::Queue, wgpu::Surface)> {
    use wgpu::TextureFormat::{Bgra8Unorm, Rgba8Unorm};

    // Create an "instance" of wgpu. This is the entry-point to the API.
    let instance = wgpu::Instance::default();

    // Create a drawable "surface" that is associated with the window.
    let surface = unsafe { instance.create_surface(&window) }?;

    // Request a GPU that is compatible with the surface. If the system has multiple GPUs then
    // pick the high performance one.
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        })
        .await
        .context("failed to find a compatible adapter")?;

    // Connect to the GPU. "device" represents the connection to the GPU and allows us to create
    // resources like buffers, textures, and pipelines. "queue" represents the command queue that
    // we use to submit commands to the GPU.
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default(), None)
        .await
        .context("failed to connect to the GPU")?;

    // Configure the texture memory backs the surface. Our renderer will draw to a surface texture
    // every frame.
    let caps = surface.get_capabilities(&adapter);
    let format = caps
        .formats
        .into_iter()
        .find(|it| matches!(it, Rgba8Unorm | Bgra8Unorm))
        .context("could not find preferred texture format (Rgba8Unorm or Bgra8Unorm)")?;
    let size = window.inner_size();
    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::AutoVsync,
        alpha_mode: caps.alpha_modes[0],
        view_formats: vec![],
    };
    surface.configure(&device, &config);

    Ok((device, queue, surface))
}
