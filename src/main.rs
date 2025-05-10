use std::ffi::CString;
use std::num::NonZeroU32;
use std::sync::Arc;

use egui_glow::Painter;
use glutin::config::ConfigTemplate;
use glutin::context::{ContextAttributesBuilder, PossiblyCurrentContext};
use glutin::display::{Display, DisplayApiPreference};
use glutin::prelude::*;
use glutin::surface::Surface;
use glutin::surface::{SurfaceAttributesBuilder, WindowSurface};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use winit::window::{Window, WindowId};

use egui_winit::State as EguiState;

#[derive(Debug)]
pub struct Viewport {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl Viewport {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Viewport {
            x,
            y,
            width,
            height,
        }
    }
}

mod graphics;
use graphics::GraphicsExample;

mod gui;
use gui::GuiExample;

#[derive(Default)]
struct App {
    window: Option<Window>,
    current_context: Option<PossiblyCurrentContext>,
    surface: Option<Surface<WindowSurface>>,
    gl: Option<Arc<glow::Context>>,
    graphics_example: Option<GraphicsExample>,
    gui: Option<GuiExample>,
    egui_context: Option<egui::Context>,
    egui_painter: Option<Painter>,
    egui_state: Option<EguiState>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Create a new window and store it in self.window
        self.window = Some(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );

        let window = self.window.as_ref().unwrap();

        // Get platform-specific handles to the display and window
        let display_handle = window.display_handle().unwrap();
        let window_handle = window.window_handle().unwrap();

        // Create a WGL (Windows OpenGL) display using the handles
        let display = unsafe {
            Display::new(
                display_handle.into(),
                DisplayApiPreference::Wgl(Some(window_handle.into())),
            )
            .expect("Failed to create Wgl display")
        };

        // Create a default OpenGL configuration
        let config_template = ConfigTemplate::default();
        let config = unsafe {
            display
                .find_configs(config_template)
                .unwrap()
                .next()
                .unwrap()
        };

        // Get the window dimensions
        let physical_size = window.inner_size();
        let width = NonZeroU32::new(physical_size.width).unwrap();
        let height = NonZeroU32::new(physical_size.height).unwrap();

        // Create attributes for the window surface
        let surface_attributes = SurfaceAttributesBuilder::<WindowSurface>::build(
            SurfaceAttributesBuilder::new(),
            window_handle.into(),
            width,
            height,
        );

        // Create context attributes (e.g., OpenGL version, flags)
        let context_attributes = ContextAttributesBuilder::new().build(Some(window_handle.into()));

        // Create the OpenGL window surface using the display and attributes
        let surface = unsafe {
            display
                .create_window_surface(&config, &surface_attributes)
                .unwrap()
        };

        // Create a non current OpenGL context
        let non_current_context = unsafe {
            display
                .create_context(&config, &context_attributes)
                .unwrap()
        };

        // Make the context current
        let current_context = non_current_context.make_current(&surface).unwrap();

        // Create the glow context
        let gl = unsafe {
            Arc::new(glow::Context::from_loader_function(|s| {
                let c_str = CString::new(s).unwrap();
                display.get_proc_address(&c_str) as *const _
            }))
        };

        self.surface = Some(surface);
        self.current_context = Some(current_context);
        self.gl = Some(gl);
        self.graphics_example = Some(GraphicsExample::new(self.gl.as_ref().unwrap()));
        self.gui = Some(GuiExample::new());
        self.egui_context = Some(egui::Context::default());
        self.egui_painter = Some(
            Painter::new(self.gl.as_ref().unwrap().clone(), "", None, false)
                .expect("Failed to create egui_glow painter"),
        );
        self.egui_state = Some(EguiState::new(
            self.egui_context.as_ref().unwrap().clone(),
            self.egui_context.as_ref().unwrap().viewport_id(),
            &window,
            Some(window.scale_factor() as f32),
            None,
            None,
        ));
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let window = self.window.as_ref().unwrap();

        // give egui any winit events
        _ = self
            .egui_state
            .as_mut()
            .unwrap()
            .on_window_event(window, &event);

        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                // Clear the the frame buffer
                self.graphics_example
                    .as_ref()
                    .unwrap()
                    .clear(self.gl.as_ref().unwrap());

                // Run the UI code
                let full_output = self.gui.as_mut().unwrap().update(
                    self.egui_state.as_mut().unwrap().take_egui_input(window),
                    self.egui_context.as_ref().unwrap(),
                );

                // Get the triangles from egui's UI
                let clipped_primitives = self
                    .egui_context
                    .as_ref()
                    .unwrap()
                    .tessellate(full_output.shapes, full_output.pixels_per_point);

                // Paint the egui UI
                let physical_size = window.inner_size();
                self.egui_painter
                    .as_mut()
                    .unwrap()
                    .paint_and_update_textures(
                        [physical_size.width, physical_size.height],
                        full_output.pixels_per_point,
                        &clipped_primitives,
                        &full_output.textures_delta,
                    );

                // Render the custom graphics code in the central panel of the ui
                self.graphics_example.as_mut().unwrap().render(
                    self.gl.as_ref().unwrap(),
                    &self.gui.as_ref().unwrap().get_viewport(window).expect(
                        "Viewport not present, make sure to update the ui before calling this",
                    ),
                    self.gui.as_ref().unwrap().get_translate(),
                    self.gui.as_ref().unwrap().get_rotate(),
                    self.gui.as_ref().unwrap().get_scale(),
                );

                // Swap the frame buffers
                self.surface
                    .as_ref()
                    .unwrap()
                    .swap_buffers(self.current_context.as_ref().unwrap())
                    .unwrap();

                window.request_redraw();
            }
            _ => (),
        }
    }
}

impl Drop for App {
    fn drop(&mut self) {
        self.egui_painter.as_mut().unwrap().destroy();
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();

    // ControlFlow::Wait pauses the event loop if no events are available to process.
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::default();

    // Add entities, components and systems to the app here

    // Run the app when behaviour is defined
    event_loop.run_app(&mut app).unwrap();
}
