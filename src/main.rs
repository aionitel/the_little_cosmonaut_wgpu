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
