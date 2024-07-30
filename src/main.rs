use winit::{
    event::{WindowEvent, KeyEvent},
    event_loop::{
        EventLoop,
        ActiveEventLoop,
    },
    window::{Window, WindowId},
    application::ApplicationHandler,
    keyboard::{KeyCode, PhysicalKey},
};
use tracing::Level;

#[derive(Default)]
struct App {
    window: Option<Window>,
}

// Packing all logic into one struct at the moment.
struct State<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: &'a Window,
}

impl<'a> State<'a> {
    async fn new(window: &'a Window) -> State<'a> {
        let size = window.inner_size();

        // Create instance of wgpu.
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        // A surface is the part of the window we draw to.
        let surface = instance.create_surface(window).unwrap();
        // Adapter is our handle to the graphics card.
        // These options will work on most devices, if wgpu can't find an adapter with the required permissions, it will return None.
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::LowPower, // Choice between LowPower and HighPerformance power usage.
                compatible_surface: Some(&surface),
                force_fallback_adapter: false, // Forces wgpu to pick adapter that will work on all(most) platforms.
            }
        ).await.unwrap();

        // Use adapter to create device.
        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(), // Required features we deem necessary for the graphics card.
                required_limits: wgpu::Limits::default(), // Limits on types and amount of resources we can create.
                label: None,
                memory_hints: wgpu::MemoryHints::MemoryUsage,
            },
            None, // No queue for now.
        ).await.unwrap();

        // Define config for surface, defines how surface creates its underlying SurfaceTexture.
        let surface_caps = surface.get_capabilities(&adapter); // Get capabilities of graphics card.
        let surface_format = surface_caps.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(event_loop.create_window(Window::default_attributes().with_title("The Little Cosmonaut")).unwrap());
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                event: KeyEvent {
                    physical_key: PhysicalKey::Code(KeyCode::Escape),
                    ..
                },
                ..
            } => {
                event_loop.exit();
            },
            _ => (),
        }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    let mut app = App::default();
    let event_loop = EventLoop::new().unwrap();

    event_loop.run_app(&mut app);
}
