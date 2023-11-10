use bevy_ecs::system::Resource;
use wgpu::{
    Device, Instance, InstanceDescriptor, Queue, RequestAdapterOptions, Surface,
    SurfaceConfiguration,
};
use winit::window::Window;

/// Context storing render surface, device, and queue
#[derive(Resource)]
pub struct RenderContext {
    surface: Surface,
    surface_config: SurfaceConfiguration,

    device: Device,
    queue: Queue,
}

impl RenderContext {
    pub async fn new(window: &Window) -> Self {
        let size = window.inner_size();

        // Create instance
        let instance = Instance::new(InstanceDescriptor::default());
        // Create surface
        let surface = unsafe {
            instance
                .create_surface(window)
                .expect("Failed to create surface")
        };

        // Create adapter
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .await
            .expect("Failed to request adapter");

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .expect("Failed to request device");

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &surface_config);

        Self {
            surface,
            surface_config,
            device,
            queue,
        }
    }
}
