use winit::{
    event::WindowEvent,
    event_loop::{
        EventLoop,
        ActiveEventLoop,
    },
    window::{Window, WindowId},
    application::ApplicationHandler,
};

#[derive(Default)]
struct App {
    window: Option<Window>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(event_loop.create_window(Window::default_attributes()).unwrap());
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            },
            _ => (),
        }
    }
}

fn main() {
    let mut app = App::default();
    let event_loop = EventLoop::new().unwrap();

    event_loop.run_app(&mut app);
}
