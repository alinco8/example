use winit::{
    application::ApplicationHandler, dpi::LogicalSize, event::WindowEvent,
    platform::web::WindowExtWebSys, window::Window,
};

#[derive(Default)]
pub struct App {
    pub window: Option<Window>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let web_window = web_sys::window().unwrap();

        let window = event_loop
            .create_window(Window::default_attributes())
            .unwrap();

        let _ = window.request_inner_size(LogicalSize::new(400.0, 200.0));
        // let _ = window.request_inner_size(PhysicalSize::new(
        //     client_window.inner_width().unwrap().as_f64().unwrap(),
        //     client_window.inner_height().unwrap().as_f64().unwrap(),
        // ));

        web_window
            .document()
            .and_then(|document| document.body())
            .and_then(|body| {
                body.append_child(&web_sys::Element::from(window.canvas().unwrap()))
                    .ok()
            })
            .unwrap();

        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            _ => (),
        };
    }
}
