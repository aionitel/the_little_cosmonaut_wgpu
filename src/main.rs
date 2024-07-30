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
    async fn new(window: &Window) -> State<'a> {
        todo!()
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

fn main() {
    tracing_subscriber::fmt().init();
    let mut app = App::default();
    let event_loop = EventLoop::new().unwrap();

    event_loop.run_app(&mut app);
}
